//! WebSocket клиент для Ren SDK
//!
//! Обрабатывает:
//! - Глобальный presence (онлайн/офлайн)
//! - События чатов (новые сообщения, typing)
//! - Отправку сообщений через WebSocket

use crate::client::RenClient;
use crate::error::SdkError;
use crate::network::api::types::messages::{Envelope, FileMetadata, Message};
use futures_util::{SinkExt, StreamExt};
use http;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};

/// Типы событий WebSocket
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WsEvent {
    /// Инициализация соединения (клиент → сервер)
    #[serde(rename = "init")]
    Init { contacts: Vec<i64> },
    
    /// Присоединиться к чату (клиент → сервер)
    #[serde(rename = "join_chat")]
    JoinChat { chat_id: i64 },
    
    /// Покинуть чат (клиент → сервер)
    #[serde(rename = "leave_chat")]
    LeaveChat { chat_id: i64 },
    
    /// Индикатор печати (клиент → сервер)
    #[serde(rename = "typing")]
    Typing { chat_id: i64, is_typing: bool },
    
    /// Отправить сообщение (клиент → сервер)
    #[serde(rename = "send_message")]
    SendMessage {
        chat_id: i64,
        message: String,
        message_type: String,
        envelopes: HashMap<String, Envelope>,
        metadata: Option<Vec<FileMetadata>>,
    },
    
    /// Presence событие (сервер → клиент)
    #[serde(rename = "presence")]
    Presence { user_id: i64, status: String },
    
    /// Новое сообщение (сервер → клиент)
    #[serde(rename = "message_new")]
    MessageNew { chat_id: i64, message: Message },
    
    /// Typing событие (сервер → клиент)
    #[serde(rename = "typing_event")]
    TypingEvent { chat_id: i64, user_id: i64, is_typing: bool },
    
    /// Успешный ответ (сервер → клиент)
    #[serde(rename = "ok")]
    Ok,
    
    /// Ошибка (сервер → клиент)
    #[serde(rename = "error")]
    Error { error: String },
}

/// WebSocket клиент
pub struct WsClient {
    client: Arc<RenClient>,
    sender: Option<mpsc::UnboundedSender<WsEvent>>,
    receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<WsEvent>>>>,
    handle: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl WsClient {
    /// Создаёт новый WebSocket клиент
    pub fn new(client: Arc<RenClient>) -> Self {
        Self {
            client,
            sender: None,
            receiver: Arc::new(RwLock::new(None)),
            handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Подключается к WebSocket серверу
    pub async fn connect(&mut self) -> Result<(), SdkError> {
        let token = self.client.get_token().await
            .ok_or(SdkError::NotAuthenticated)?;
        
        let base_url = self.client.base_url();
        let ws_url = base_url
            .replace("http://", "ws://")
            .replace("https://", "wss://");
        
        let url_str = format!("{}/ws", ws_url);
        let url: http::Uri = url_str.parse()
            .map_err(|e| SdkError::WebSocket(format!("Invalid URL: {}", e)))?;
        
        let request = http::Request::builder()
            .uri(&url)
            .header("Authorization", format!("Bearer {}", token))
            .body(())?;
        
        let (ws_stream, _) = connect_async(request).await
            .map_err(|e| SdkError::WebSocket(format!("Connection error: {}", e)))?;
        
        let (mut write, mut read) = ws_stream.split();
        
        let (tx, mut rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        self.sender = Some(tx);
        *self.receiver.write().await = Some(event_rx);
        
        let handle = tokio::spawn(async move {
            // Задача для отправки сообщений
            let mut send_task = {
                let mut rx_clone = rx;
                tokio::spawn(async move {
                    while let Some(event) = rx_clone.recv().await {
                        let json = match serde_json::to_string(&event) {
                            Ok(j) => j,
                            Err(e) => {
                                log::error!("Failed to serialize event: {}", e);
                                continue;
                            }
                        };
                        if let Err(e) = write.send(WsMessage::Text(json)).await {
                            log::error!("Failed to send message: {}", e);
                            break;
                        }
                    }
                })
            };
            
            // Задача для приёма сообщений
            let mut recv_task = {
                let event_tx_clone = event_tx;
                tokio::spawn(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(WsMessage::Text(text)) => {
                                match serde_json::from_str::<WsEvent>(&text) {
                                    Ok(event) => {
                                        if event_tx_clone.send(event).is_err() {
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to parse event: {}", e);
                                    }
                                }
                            }
                            Ok(WsMessage::Close(_)) => {
                                break;
                            }
                            Err(e) => {
                                log::error!("WebSocket error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                })
            };
            
            tokio::select! {
                _ = &mut send_task => {},
                _ = &mut recv_task => {},
            }
        });
        
        *self.handle.write().await = Some(handle);
        
        Ok(())
    }

    /// Отключается от WebSocket сервера
    pub async fn disconnect(&mut self) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(WsEvent::LeaveChat { chat_id: 0 }); // Отправляем фиктивное событие для закрытия
        }
        self.sender = None;
        *self.receiver.write().await = None;
        if let Some(handle) = self.handle.write().await.take() {
            handle.abort();
        }
    }

    /// Инициализирует соединение с контактами
    pub async fn init(&self, contacts: Vec<i64>) -> Result<(), SdkError> {
        if let Some(sender) = &self.sender {
            sender.send(WsEvent::Init { contacts })
                .map_err(|_| SdkError::WebSocket("Failed to send init event".to_string()))?;
            Ok(())
        } else {
            Err(SdkError::WebSocket("Not connected".to_string()))
        }
    }

    /// Присоединяется к чату
    pub async fn join_chat(&self, chat_id: i64) -> Result<(), SdkError> {
        if let Some(sender) = &self.sender {
            sender.send(WsEvent::JoinChat { chat_id })
                .map_err(|_| SdkError::WebSocket("Failed to send join_chat event".to_string()))?;
            Ok(())
        } else {
            Err(SdkError::WebSocket("Not connected".to_string()))
        }
    }

    /// Покидает чат
    pub async fn leave_chat(&self, chat_id: i64) -> Result<(), SdkError> {
        if let Some(sender) = &self.sender {
            sender.send(WsEvent::LeaveChat { chat_id })
                .map_err(|_| SdkError::WebSocket("Failed to send leave_chat event".to_string()))?;
            Ok(())
        } else {
            Err(SdkError::WebSocket("Not connected".to_string()))
        }
    }

    /// Отправляет индикатор печати
    pub async fn send_typing(&self, chat_id: i64, is_typing: bool) -> Result<(), SdkError> {
        if let Some(sender) = &self.sender {
            sender.send(WsEvent::Typing { chat_id, is_typing })
                .map_err(|_| SdkError::WebSocket("Failed to send typing event".to_string()))?;
            Ok(())
        } else {
            Err(SdkError::WebSocket("Not connected".to_string()))
        }
    }

    /// Отправляет сообщение через WebSocket
    pub async fn send_message(
        &self,
        chat_id: i64,
        message: String,
        message_type: String,
        envelopes: HashMap<String, Envelope>,
        metadata: Option<Vec<FileMetadata>>,
    ) -> Result<(), SdkError> {
        if let Some(sender) = &self.sender {
            sender.send(WsEvent::SendMessage {
                chat_id,
                message,
                message_type,
                envelopes,
                metadata,
            })
            .map_err(|_| SdkError::WebSocket("Failed to send message".to_string()))?;
            Ok(())
        } else {
            Err(SdkError::WebSocket("Not connected".to_string()))
        }
    }

    /// Получает следующее событие (блокирующий вызов)
    pub async fn next_event(&self) -> Option<WsEvent> {
        let mut receiver = self.receiver.write().await;
        if let Some(ref mut rx) = *receiver {
            rx.recv().await
        } else {
            None
        }
    }

    /// Проверяет, подключен ли клиент
    pub fn is_connected(&self) -> bool {
        self.sender.is_some()
    }
}

