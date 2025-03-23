# Tutorial: Criando um Chat em Tempo Real com WebSockets em Rust

Neste tutorial, vamos criar um sistema de chat em tempo real utilizando WebSockets com Rust no backend e JavaScript no frontend. O projeto demonstra como implementar comunicação bidirecional em tempo real entre clientes e servidor.

## Passo 1: Configurando o Projeto

Primeiro, crie uma nova pasta para o projeto e inicialize um projeto Rust:

```bash
mkdir app04
cd app04
cargo init
```

## Passo 2: Configurando as Dependências

Edite o arquivo `Cargo.toml` para adicionar as dependências necessárias:

```toml
[package]
name = "app04"
version = "0.1.0"
edition = "2021"

[dependencies]
actix = "0.13.1"
actix-web = "4.4.0"
actix-web-actors = "4.2.0"
actix-cors = "0.6.4"
actix-files = "0.6.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.4", features = ["v4", "serde"] }
env_logger = "0.10.0"
log = "0.4"
```

Aqui estamos adicionando:
- `actix`: Framework de atores para Rust
- `actix-web`: Framework web
- `actix-web-actors`: Integração de WebSockets com Actix
- `actix-cors`: Para configurar CORS
- `actix-files`: Para servir arquivos estáticos
- `serde` e `serde_json`: Para serialização/deserialização
- `uuid`: Para gerar IDs únicos
- `env_logger` e `log`: Para logging

## Passo 3: Criando a Estrutura do Projeto

Crie a pasta para os arquivos estáticos:

```bash
mkdir -p static
```

## Passo 4: Implementando o Servidor de Chat

Vamos começar implementando o servidor no arquivo `src/main.rs`. Primeiro, vamos importar as dependências e definir as estruturas básicas:

```rust
use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Estrutura para armazenar as mensagens
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    id: String,
    username: String,
    message: String,
    timestamp: u64,
}

// Estrutura para armazenar os clientes conectados
struct ChatServer {
    sessions: HashMap<String, Addr<ChatSession>>,
    messages: Vec<ChatMessage>,
}
```

## Passo 5: Implementando o Servidor de Chat (Métodos)

Agora, vamos implementar os métodos para o servidor de chat:

```rust
impl ChatServer {
    fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            messages: Vec::new(),
        }
    }

    // Método para enviar mensagem para todos os clientes conectados
    fn broadcast_message(&mut self, message: ChatMessage) {
        self.messages.push(message.clone());
        
        // Envia a mensagem para todos os clientes
        for (_id, addr) in &self.sessions {
            addr.do_send(WsMessage(serde_json::to_string(&message).unwrap()));
        }
    }
}

// Actor para o servidor de chat
impl Actor for ChatServer {
    type Context = actix::Context<Self>;
}
```

## Passo 6: Definindo os Tipos de Mensagens

Agora, vamos definir os tipos de mensagens que serão trocadas entre os atores:

```rust
// Mensagem para registrar uma nova sessão
#[derive(Message)]
#[rtype(result = "()")]
struct Connect {
    id: String,
    addr: Addr<ChatSession>,
}

// Mensagem para remover uma sessão
#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    id: String,
}

// Mensagem de chat
#[derive(Message)]
#[rtype(result = "()")]
struct ClientMessage {
    id: String,
    msg: String,
    username: String,
}

// Mensagem para enviar ao cliente
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);
```

## Passo 7: Implementando Handlers para as Mensagens

Agora, vamos implementar os handlers para cada tipo de mensagem:

```rust
// Handler para registrar nova sessão
impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        // Adiciona a nova sessão
        self.sessions.insert(msg.id.clone(), msg.addr);
        
        // Envia histórico de mensagens para o novo cliente
        if let Some(addr) = self.sessions.get(&msg.id) {
            let msgs = self.messages.clone();
            for msg in msgs {
                addr.do_send(WsMessage(serde_json::to_string(&msg).unwrap()));
            }
        }
    }
}

// Handler para remover sessão
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

// Handler para mensagens de chat
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        // Cria uma nova mensagem de chat
        let chat_message = ChatMessage {
            id: Uuid::new_v4().to_string(),
            username: msg.username,
            message: msg.msg,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        // Envia a mensagem para todos os clientes
        self.broadcast_message(chat_message);
    }
}
```

## Passo 8: Implementando a Sessão de Chat

Agora, vamos implementar a estrutura e comportamento da sessão de chat:

```rust
// Define a estrutura da sessão de chat
struct ChatSession {
    id: String,
    username: String,
    server: Arc<Mutex<ChatServer>>,
}

// Implementa o actor para a sessão
impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Registra a sessão com o servidor
        let addr = ctx.address();
        self.server
            .lock()
            .unwrap()
            .sessions
            .insert(self.id.clone(), addr.clone());

        // Notifica o servidor sobre a nova conexão
        let mut server = self.server.lock().unwrap();
        server.broadcast_message(ChatMessage {
            id: Uuid::new_v4().to_string(),
            username: "sistema".to_string(),
            message: format!("{} entrou no chat", self.username),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        // Notifica o servidor que a sessão está sendo finalizada
        let mut server = self.server.lock().unwrap();
        server.sessions.remove(&self.id);
        server.broadcast_message(ChatMessage {
            id: Uuid::new_v4().to_string(),
            username: "sistema".to_string(),
            message: format!("{} saiu do chat", self.username),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        actix::Running::Stop
    }
}

// Handler para mensagens WebSocket
impl Handler<WsMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
```

## Passo 9: Implementando o Handler de WebSocket

Agora, vamos implementar o handler para as mensagens WebSocket:

```rust
// Estrutura de dados para mensagens WebSocket
#[derive(Deserialize)]
struct WebSocketMessage {
    #[serde(default)]
    action: String,
    #[serde(default)]
    message: String,
    #[serde(default)]
    username: String,
}

// Implementa o handler para mensagens WebSocket
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg_result = serde_json::from_str::<WebSocketMessage>(&text);
                
                if let Ok(msg) = msg_result {
                    match msg.action.as_str() {
                        "message" => {
                            // Se não tiver um nome de usuário configurado, use um anônimo
                            if self.username.is_empty() {
                                self.username = "Anônimo".to_string();
                            }
                            
                            // Envia a mensagem para o servidor
                            let message = ClientMessage {
                                id: self.id.clone(),
                                msg: msg.message,
                                username: self.username.clone(),
                            };
                            
                            self.server.lock().unwrap().broadcast_message(ChatMessage {
                                id: Uuid::new_v4().to_string(),
                                username: self.username.clone(),
                                message: message.msg.clone(),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            });
                        }
                        "setUsername" => {
                            let old_name = self.username.clone();
                            self.username = msg.username;
                            
                            // Notifica a mudança de nome se não for a primeira definição
                            if !old_name.is_empty() && old_name != self.username {
                                self.server.lock().unwrap().broadcast_message(ChatMessage {
                                    id: Uuid::new_v4().to_string(),
                                    username: "sistema".to_string(),
                                    message: format!("{} agora é conhecido como {}", old_name, self.username),
                                    timestamp: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                });
                            }
                        }
                        _ => {
                            log::warn!("Ação desconhecida: {}", msg.action);
                        }
                    }
                } else {
                    log::error!("Falha ao parsear mensagem: {:?}", text);
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
```

## Passo 10: Criando Handlers HTTP

Agora, vamos criar os handlers HTTP para conectar via WebSocket e servir a página inicial:

```rust
// Handler para estabelecer a conexão WebSocket
#[get("/ws/{username}")]
async fn chat_ws(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    server: web::Data<Arc<Mutex<ChatServer>>>,
) -> Result<HttpResponse, Error> {
    let username = path.into_inner();
    let session_id = Uuid::new_v4().to_string();
    
    ws::start(
        ChatSession {
            id: session_id,
            username,
            server: server.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// Rota para a página inicial
#[get("/")]
async fn index() -> impl Responder {
    fs::NamedFile::open_async("./static/index.html").await.unwrap()
}
```

## Passo 11: Configurando o Servidor HTTP

Finalmente, vamos implementar a função `main` para inicializar e configurar o servidor:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // Cria o servidor de chat
    let chat_server = Arc::new(Mutex::new(ChatServer::new()));
    
    println!("Servidor iniciado em http://127.0.0.1:8080");
    println!("WebSocket disponível em ws://127.0.0.1:8080/ws/seuNome");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(index)
            .service(chat_ws)
            .service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Parte 2: Implementando a Interface do Cliente

Agora que implementamos o servidor WebSocket, vamos criar uma interface amigável para o chat utilizando HTML, CSS e JavaScript.

### Passo 1: Criando a Estrutura HTML

Crie um arquivo chamado `static/index.html` com o seguinte conteúdo:

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chat em Tempo Real com Rust WebSockets</title>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        }
        
        body {
            background-color: #f5f5f5;
            display: flex;
            flex-direction: column;
            height: 100vh;
        }
        
        .chat-container {
            max-width: 800px;
            margin: 20px auto;
            background-color: white;
            border-radius: 10px;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
            display: flex;
            flex-direction: column;
            flex-grow: 1;
            overflow: hidden;
        }
        
        .chat-header {
            background-color: #4a76a8;
            color: white;
            padding: 15px 20px;
            border-radius: 10px 10px 0 0;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        
        .chat-header h1 {
            font-size: 1.5rem;
        }
        
        .connection-status {
            font-size: 0.9rem;
            display: flex;
            align-items: center;
        }
        
        .status-indicator {
            width: 12px;
            height: 12px;
            border-radius: 50%;
            margin-right: 5px;
        }
        
        .connected {
            background-color: #4CAF50;
        }
        
        .disconnected {
            background-color: #F44336;
        }
        
        .chat-messages {
            flex-grow: 1;
            padding: 20px;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        
        .message {
            padding: 10px 15px;
            border-radius: 18px;
            max-width: 70%;
            word-break: break-word;
        }
        
        .message-container {
            display: flex;
            flex-direction: column;
        }
        
        .message-info {
            font-size: 0.8rem;
            margin-bottom: 2px;
            color: #555;
        }
        
        .message-text {
            font-size: 1rem;
        }
        
        .user-message {
            align-self: flex-end;
            background-color: #e3f2fd;
            border-bottom-right-radius: 5px;
        }
        
        .other-message {
            align-self: flex-start;
            background-color: #f1f1f1;
            border-bottom-left-radius: 5px;
        }
        
        .system-message {
            align-self: center;
            background-color: #fff3cd;
            color: #856404;
            font-style: italic;
            padding: 8px 15px;
            border-radius: 20px;
            max-width: 80%;
            text-align: center;
        }
        
        .chat-input {
            display: flex;
            padding: 15px;
            background-color: #f9f9f9;
            border-top: 1px solid #eee;
        }
        
        .chat-input input {
            flex-grow: 1;
            padding: 12px 15px;
            border: 1px solid #ddd;
            border-radius: 25px;
            font-size: 1rem;
            outline: none;
        }
        
        .chat-input input:focus {
            border-color: #4a76a8;
        }
        
        .chat-input button {
            margin-left: 10px;
            padding: 12px 20px;
            background-color: #4a76a8;
            color: white;
            border: none;
            border-radius: 25px;
            cursor: pointer;
            transition: background-color 0.2s;
            font-weight: bold;
        }
        
        .chat-input button:hover {
            background-color: #3b5998;
        }
        
        .chat-input button:disabled {
            background-color: #cccccc;
            cursor: not-allowed;
        }
        
        .user-form {
            padding: 20px;
            background-color: white;
            border-radius: 10px;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
            max-width: 400px;
            margin: 100px auto;
        }
        
        .user-form h2 {
            margin-bottom: 20px;
            color: #333;
        }
        
        .user-form input {
            width: 100%;
            padding: 12px 15px;
            margin-bottom: 15px;
            border: 1px solid #ddd;
            border-radius: 5px;
            font-size: 1rem;
            outline: none;
        }
        
        .user-form button {
            width: 100%;
            padding: 12px;
            background-color: #4a76a8;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-size: 1rem;
            transition: background-color 0.2s;
        }
        
        .user-form button:hover {
            background-color: #3b5998;
        }

        @media (max-width: 768px) {
            .chat-container {
                margin: 10px;
                height: calc(100vh - 20px);
            }
            
            .message {
                max-width: 85%;
            }
        }
    </style>
</head>
<body>
    <!-- Formulário de entrada de nome de usuário -->
    <div id="userForm" class="user-form">
        <h2>Entre no Chat</h2>
        <input type="text" id="usernameInput" placeholder="Seu nome ou apelido" maxlength="20">
        <button id="joinButton">Entrar no Chat</button>
    </div>

    <!-- Container principal do chat (inicialmente oculto) -->
    <div id="chatContainer" class="chat-container" style="display: none;">
        <div class="chat-header">
            <h1>Chat em Tempo Real</h1>
            <div class="connection-status">
                <div id="statusIndicator" class="status-indicator disconnected"></div>
                <span id="statusText">Desconectado</span>
            </div>
        </div>
        
        <div id="chatMessages" class="chat-messages">
            <!-- As mensagens serão inseridas aqui dinamicamente -->
        </div>
        
        <div class="chat-input">
            <input type="text" id="messageInput" placeholder="Digite sua mensagem..." disabled>
            <button id="sendButton" disabled>Enviar</button>
        </div>
    </div>

    <script src="script.js"></script>
</body>
</html>
```

### Passo 2: Implementando o JavaScript do Cliente

Crie um arquivo chamado `static/script.js` com o seguinte conteúdo:

```javascript
document.addEventListener('DOMContentLoaded', function() {
    // Elementos do DOM
    const userForm = document.getElementById('userForm');
    const usernameInput = document.getElementById('usernameInput');
    const joinButton = document.getElementById('joinButton');
    const chatContainer = document.getElementById('chatContainer');
    const messageInput = document.getElementById('messageInput');
    const sendButton = document.getElementById('sendButton');
    const chatMessages = document.getElementById('chatMessages');
    const statusIndicator = document.getElementById('statusIndicator');
    const statusText = document.getElementById('statusText');
    
    // Variáveis para WebSocket e nome de usuário
    let socket = null;
    let username = '';
    
    // Função para criar e configurar a conexão WebSocket
    function connectWebSocket(username) {
        // Determinar a URL do servidor WebSocket
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws/${username}`;
        
        // Criar a conexão WebSocket
        socket = new WebSocket(wsUrl);
        
        // Configurar os manipuladores de eventos do WebSocket
        socket.onopen = function() {
            // Atualizar a UI para mostrar que estamos conectados
            statusIndicator.classList.replace('disconnected', 'connected');
            statusText.textContent = 'Conectado';
            
            // Habilitar a entrada de mensagens
            messageInput.disabled = false;
            sendButton.disabled = false;
            messageInput.focus();
            
            // Adicionar mensagem de sistema
            addSystemMessage('Você entrou no chat!');
        };
        
        socket.onmessage = function(event) {
            // Processar mensagem recebida
            const message = JSON.parse(event.data);
            
            // Verificar o tipo de mensagem e agir de acordo
            if (message.type === 'connect') {
                // Novo usuário entrou no chat
                addSystemMessage(`${message.username} entrou no chat!`);
            } else if (message.type === 'disconnect') {
                // Usuário saiu do chat
                addSystemMessage(`${message.username} saiu do chat!`);
            } else if (message.type === 'message') {
                // Mensagem normal de chat
                addChatMessage(message.username, message.content, message.username === username);
            }
        };
        
        socket.onclose = function() {
            // Atualizar a UI para mostrar que estamos desconectados
            statusIndicator.classList.replace('connected', 'disconnected');
            statusText.textContent = 'Desconectado';
            
            // Desabilitar a entrada de mensagens
            messageInput.disabled = true;
            sendButton.disabled = true;
            
            // Adicionar mensagem de sistema
            addSystemMessage('Você foi desconectado do chat. Tente entrar novamente.');
            
            // Opcional: tentar reconectar automaticamente após um tempo
            setTimeout(function() {
                if (username) {
                    connectWebSocket(username);
                }
            }, 5000);
        };
        
        socket.onerror = function(error) {
            console.error('Erro na conexão WebSocket:', error);
            addSystemMessage('Erro de conexão. Por favor, tente novamente mais tarde.');
        };
    }
    
    // Função para adicionar mensagem de sistema ao chat
    function addSystemMessage(text) {
        const messageElement = document.createElement('div');
        messageElement.className = 'system-message';
        messageElement.textContent = text;
        
        chatMessages.appendChild(messageElement);
        scrollToBottom();
    }
    
    // Função para adicionar mensagem de chat ao chat
    function addChatMessage(username, text, isOwnMessage) {
        const messageContainer = document.createElement('div');
        messageContainer.className = 'message-container';
        
        const messageInfo = document.createElement('div');
        messageInfo.className = 'message-info';
        messageInfo.textContent = isOwnMessage ? 'Você' : username;
        
        const messageElement = document.createElement('div');
        messageElement.className = `message ${isOwnMessage ? 'user-message' : 'other-message'}`;
        
        const messageText = document.createElement('div');
        messageText.className = 'message-text';
        messageText.textContent = text;
        
        messageElement.appendChild(messageText);
        messageContainer.appendChild(messageInfo);
        messageContainer.appendChild(messageElement);
        
        chatMessages.appendChild(messageContainer);
        scrollToBottom();
    }
    
    // Função para rolar automaticamente para o final do chat
    function scrollToBottom() {
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }
    
    // Função para enviar mensagem
    function sendMessage() {
        const text = messageInput.value.trim();
        
        if (text && socket && socket.readyState === WebSocket.OPEN) {
            // Enviar a mensagem para o servidor
            const message = {
                type: 'message',
                content: text
            };
            
            socket.send(JSON.stringify(message));
            
            // Limpar o campo de entrada
            messageInput.value = '';
            messageInput.focus();
        }
    }
    
    // Event listeners
    joinButton.addEventListener('click', function() {
        username = usernameInput.value.trim();
        
        if (username) {
            // Esconder o formulário de entrada e mostrar o chat
            userForm.style.display = 'none';
            chatContainer.style.display = 'flex';
            
            // Conectar ao WebSocket
            connectWebSocket(username);
        }
    });
    
    // Também permitir pressionar Enter para entrar no chat
    usernameInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            joinButton.click();
        }
    });
    
    sendButton.addEventListener('click', sendMessage);
    
    // Também permitir pressionar Enter para enviar mensagem
    messageInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            sendMessage();
        }
    });
    
    // Tentar reconectar quando a página ficar visível novamente
    document.addEventListener('visibilitychange', function() {
        if (document.visibilityState === 'visible' && username && (!socket || socket.readyState !== WebSocket.OPEN)) {
            connectWebSocket(username);
        }
    });
    
    // Configurar para tentar reconectar quando a janela for fechada
    window.addEventListener('beforeunload', function() {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.close();
        }
    });
});
```

### Passo 3: Executando e Testando

Agora que criamos nosso cliente e servidor, vamos testá-los juntos:

1. Abra um terminal no diretório do projeto
2. Execute o servidor com o comando:
   ```
   cargo run
   ```
3. Abra seu navegador e acesse `http://localhost:8080`
4. Digite um nome de usuário e entre no chat
5. Para testar o chat em tempo real, abra o mesmo endereço em outras abas ou navegadores e entre com nomes diferentes

### Funcionalidades Implementadas

Nosso sistema de chat agora possui:

1. **Conexão WebSocket Persistente**: Comunicação bidirecional em tempo real entre clientes e servidor
2. **Interface Amigável**: Um design moderno e responsivo para o chat
3. **Notificações de Status**: Mensagens do sistema informando quando usuários entram ou saem
4. **Indicador de Conexão**: Mostra visualmente o estado da conexão do usuário
5. **Reconexão Automática**: Tenta restabelecer a conexão caso ela seja perdida
6. **Suporte para Múltiplos Usuários**: Vários usuários podem se conectar e conversar simultaneamente

## Conclusão

Parabéns! Você criou com sucesso um sistema de chat em tempo real usando WebSockets com Rust no servidor e JavaScript no cliente. Este projeto demonstra:

1. Como implementar comunicação bidirecional em tempo real
2. Como usar atores para gerenciar conexões concorrentes
3. Como criar uma interface web interativa para WebSockets
4. Como lidar com diferentes tipos de mensagens entre servidor e cliente

Este projeto serve como uma base sólida que você pode expandir com recursos adicionais, como:

- Salas de chat privadas
- Suporte para envio de imagens ou arquivos
- Persistência de mensagens em banco de dados
- Notificações de "digitando..."
- Emojis e formatação de texto

No próximo projeto, vamos criar uma aplicação CRUD completa com um frontend React! 