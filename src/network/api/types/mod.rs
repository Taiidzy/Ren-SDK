//! Типы данных для всех API эндпоинтов

pub mod auth;
pub mod users;
pub mod chats;
pub mod messages;

pub use auth::{AuthError, LoginRequest, LoginResponse, RegisterRequest};
pub use users::{PublicKeyResponse, UpdateUsernameRequest, UserResponse};
pub use chats::{Chat, CreateChatRequest};
pub use messages::{Envelope, FileMetadata, Message};

