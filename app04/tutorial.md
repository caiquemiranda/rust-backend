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

## Passo 12: Criando a Interface do Cliente

Agora, vamos criar a interface do cliente em `static/index.html`:

```html
<!DOCTYPE html>
<html lang="pt-br">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chat em Tempo Real com Rust e WebSockets</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            background-color: #f4f7f9;
        }
        h1 {
            color: #2c3e50;
            text-align: center;
        }
        .container {
            display: flex;
            flex-direction: column;
            height: 80vh;
        }
        .chat-box {
            flex-grow: 1;
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
            overflow-y: auto;
            padding: 15px;
            margin-bottom: 20px;
        }
        .input-area {
            display: flex;
            margin-bottom: 10px;
        }
        input, button {
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        input {
            flex-grow: 1;
            margin-right: 10px;
        }
        button {
            background-color: #3498db;
            color: white;
            border: none;
            cursor: pointer;
            transition: background-color 0.3s;
        }
        button:hover {
            background-color: #2980b9;
        }
        .message {
            margin-bottom: 10px;
            padding: 10px;
            border-radius: 5px;
        }
        .user-message {
            background-color: #e8f4f8;
            text-align: right;
        }
        .other-message {
            background-color: #f0f0f0;
        }
        .system-message {
            background-color: #f8f8e8;
            font-style: italic;
            text-align: center;
        }
        .username-form {
            margin-bottom: 20px;
            padding: 15px;
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
        }
        .message-header {
            font-weight: bold;
            margin-bottom: 5px;
        }
        .timestamp {
            font-size: 0.8em;
            color: #7f8c8d;
        }
    </style>
</head>
<body>
    <h1>Chat em Tempo Real</h1>
    
    <div class="username-form">
        <div class="input-area">
            <input type="text" id="username-input" placeholder="Digite seu nome de usuário">
            <button id="set-username-btn">Definir Nome</button>
        </div>
    </div>
    
    <div class="container">
        <div id="chat-box" class="chat-box"></div>
        
        <div class="input-area">
            <input type="text" id="message-input" placeholder="Digite sua mensagem">
            <button id="send-btn">Enviar</button>
        </div>
    </div>
    
    <script>
        let ws;
        let currentUsername = '';
        
        // Elementos da DOM
        const chatBox = document.getElementById('chat-box');
        const messageInput = document.getElementById('message-input');
        const sendBtn = document.getElementById('send-btn');
        const usernameInput = document.getElementById('username-input');
        const setUsernameBtn = document.getElementById('set-username-btn');
        
        // Função para formatar a data
        function formatTimestamp(timestamp) {
            const date = new Date(timestamp * 1000);
            return date.toLocaleTimeString();
        }
        
        // Conectar ao WebSocket
        function connectWebSocket(username) {
            // Fecha a conexão anterior, se houver
            if (ws) {
                ws.close();
            }
            
            ws = new WebSocket(`ws://${window.location.host}/ws/${username}`);
            
            ws.onopen = () => {
                console.log('Conectado ao WebSocket');
                
                // Se for a primeira conexão, envia o nome de usuário
                if (currentUsername !== username) {
                    currentUsername = username;
                    sendWebSocketMessage('setUsername', '', username);
                }
            };
            
            ws.onmessage = (event) => {
                const data = JSON.parse(event.data);
                displayMessage(data);
            };
            
            ws.onclose = () => {
                console.log('Desconectado do WebSocket');
                // Tenta reconectar após 3 segundos
                setTimeout(() => {
                    connectWebSocket(currentUsername || 'Anônimo');
                }, 3000);
            };
            
            ws.onerror = (error) => {
                console.error('Erro no WebSocket:', error);
            };
        }
        
        // Enviar mensagem para o WebSocket
        function sendWebSocketMessage(action, message = '', username = '') {
            if (ws && ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify({
                    action,
                    message,
                    username
                }));
            }
        }
        
        // Exibir mensagem no chat
        function displayMessage(data) {
            const messageDiv = document.createElement('div');
            messageDiv.className = 'message';
            
            // Adicionar classe com base no tipo de mensagem
            if (data.username === 'sistema') {
                messageDiv.classList.add('system-message');
            } else if (data.username === currentUsername) {
                messageDiv.classList.add('user-message');
            } else {
                messageDiv.classList.add('other-message');
            }
            
            // Criar cabeçalho da mensagem
            if (data.username !== 'sistema') {
                const header = document.createElement('div');
                header.className = 'message-header';
                header.textContent = data.username;
                messageDiv.appendChild(header);
            }
            
            // Conteúdo da mensagem
            const content = document.createElement('div');
            content.textContent = data.message;
            messageDiv.appendChild(content);
            
            // Timestamp
            const timestamp = document.createElement('div');
            timestamp.className = 'timestamp';
            timestamp.textContent = formatTimestamp(data.timestamp);
            messageDiv.appendChild(timestamp);
            
            chatBox.appendChild(messageDiv);
            
            // Rolar para a mensagem mais recente
            chatBox.scrollTop = chatBox.scrollHeight;
        }
        
        // Eventos
        sendBtn.addEventListener('click', () => {
            const message = messageInput.value.trim();
            if (message) {
                sendWebSocketMessage('message', message);
                messageInput.value = '';
            }
        });
        
        messageInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                sendBtn.click();
            }
        });
        
        setUsernameBtn.addEventListener('click', () => {
            const username = usernameInput.value.trim();
            if (username) {
                if (!currentUsername) {
                    // Primeira definição do nome de usuário
                    connectWebSocket(username);
                } else {
                    // Mudança de nome de usuário
                    sendWebSocketMessage('setUsername', '', username);
                    currentUsername = username;
                }
            }
        });
        
        usernameInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                setUsernameBtn.click();
            }
        });
        
        // Inicialização
        window.addEventListener('DOMContentLoaded', () => {
            // Iniciar com um nome aleatório
            usernameInput.value = `Usuário${Math.floor(Math.random() * 1000)}`;
            setUsernameBtn.click();
        });
    </script>
</body>
</html>
```

## Passo 13: Executando e Testando

Para executar o servidor:

```bash
cargo run
```

Agora, abra seu navegador e acesse:

```
http://localhost:8080
```

Para testar o chat com múltiplos usuários:
1. Abra várias janelas do navegador apontando para http://localhost:8080
2. Defina nomes de usuários diferentes em cada janela
3. Envie mensagens e observe a comunicação em tempo real entre as janelas

## Conclusão

Parabéns! Você criou um sistema de chat em tempo real usando WebSockets com Rust e JavaScript. Este projeto demonstra:

1. Como implementar comunicação bidirecional em tempo real entre cliente e servidor
2. Como utilizar o modelo de ator do Actix para gerenciar estado e sessões
3. Como transmitir mensagens para múltiplos clientes conectados
4. Como criar uma interface interativa para aplicações em tempo real

No próximo projeto, vamos ampliar nosso conhecimento e criar um sistema completo com frontend React e backend Rust, integrando várias das técnicas que aprendemos até agora. 