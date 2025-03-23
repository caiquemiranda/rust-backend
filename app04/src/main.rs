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

// Mensagem para enviar ao cliente
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

// Handler para mensagens WebSocket
impl Handler<WsMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

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