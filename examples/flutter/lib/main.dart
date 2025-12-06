import 'package:flutter/material.dart';
import 'ren_sdk_simple.dart' as ren_sdk;

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Ren SDK Flutter Example',
      theme: ThemeData(primarySwatch: Colors.blue),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  ren_sdk.RenSDK? _sdk;
  String _status = 'Не инициализирован';
  Map<String, dynamic>? _profile;
  List<Map<String, dynamic>>? _chats;

  @override
  void initState() {
    super.initState();
    _initSDK();
  }

  void _initSDK() {
    try {
      _sdk = ren_sdk.RenSDK.create('http://localhost:8001');
      setState(() {
        _status = 'Клиент инициализирован';
      });
    } catch (e) {
      setState(() {
        _status = 'Ошибка инициализации: $e';
      });
    }
  }

  Future<void> _login() async {
    if (_sdk == null) return;

    try {
      _sdk!.login('user123', 'password', rememberMe: false);
      setState(() {
        _status = 'Вход выполнен успешно';
      });
      _loadProfile();
      _loadChats();
    } catch (e) {
      setState(() {
        _status = 'Ошибка входа: $e';
      });
    }
  }

  Future<void> _loadProfile() async {
    if (_sdk == null) return;

    try {
      final profile = _sdk!.getMe();
      setState(() {
        _profile = profile;
      });
    } catch (e) {
      setState(() {
        _status = 'Ошибка загрузки профиля: $e';
      });
    }
  }

  Future<void> _loadChats() async {
    if (_sdk == null) return;

    try {
      final chats = _sdk!.getChats();
      setState(() {
        _chats = chats;
      });
    } catch (e) {
      setState(() {
        _status = 'Ошибка загрузки чатов: $e';
      });
    }
  }

  void _generateKeypair() {
    try {
      final keypair = ren_sdk.RenSDKCrypto.generateKeypair();
      showDialog(
        context: context,
        builder: (context) => AlertDialog(
          title: const Text('Сгенерированная пара ключей'),
          content: SingleChildScrollView(
            child: Text(
              'Public Key: ${keypair['public_key']}\n\n'
              'Private Key: ${keypair['private_key']}',
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: const Text('OK'),
            ),
          ],
        ),
      );
    } catch (e) {
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Ошибка: $e')));
    }
  }

  @override
  void dispose() {
    _sdk?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Ren SDK Example')),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      'Статус',
                      style: TextStyle(
                        fontSize: 18,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                    const SizedBox(height: 8),
                    Text(_status),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 16),
            ElevatedButton(onPressed: _login, child: const Text('Войти')),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _generateKeypair,
              child: const Text('Сгенерировать пару ключей'),
            ),
            if (_profile != null) ...[
              const SizedBox(height: 16),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Профиль',
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      Text('ID: ${_profile!['id']}'),
                      Text('Логин: ${_profile!['login']}'),
                      Text('Имя: ${_profile!['username']}'),
                    ],
                  ),
                ),
              ),
            ],
            if (_chats != null) ...[
              const SizedBox(height: 16),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Чаты (${_chats!.length})',
                        style: TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                      const SizedBox(height: 8),
                      ..._chats!.map(
                        (chat) => ListTile(
                          title: Text(chat['peer_username'] ?? 'Unknown'),
                          subtitle: Text('ID: ${chat['id']}'),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
