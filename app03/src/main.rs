use actix_web::{get, post, put, delete, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use dotenv::dotenv;
use std::env;

// Estrutura para representar uma tarefa, derivada para deserialização, serialização e mapeamento de linha SQL
#[derive(Serialize, Deserialize, FromRow, Debug)]
struct Tarefa {
    #[serde(skip_deserializing)]
    id: Option<i64>,
    titulo: String,
    descricao: String,
    concluida: bool,
}

// Estrutura para resposta de API padronizada
#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

// Handler para rota principal
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Tarefas com SQLite. Use /tarefas para gerenciar suas tarefas.")
}

// Handler para listar todas as tarefas
#[get("/tarefas")]
async fn listar_tarefas(db: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Tarefa>("SELECT * FROM tarefas")
        .fetch_all(db.get_ref())
        .await
    {
        Ok(tarefas) => {
            let response = ApiResponse {
                status: "sucesso".to_string(),
                data: tarefas,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Erro ao listar tarefas: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Handler para obter uma tarefa por ID
#[get("/tarefas/{id}")]
async fn obter_tarefa(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query_as::<_, Tarefa>("SELECT * FROM tarefas WHERE id = ?")
        .bind(id)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(Some(tarefa)) => {
            let response = ApiResponse {
                status: "sucesso".to_string(),
                data: tarefa,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Tarefa com ID {} não encontrada", id),
            };
            HttpResponse::NotFound().json(response)
        }
        Err(e) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Erro ao buscar tarefa: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Handler para criar uma nova tarefa
#[post("/tarefas")]
async fn criar_tarefa(db: web::Data<SqlitePool>, tarefa: web::Json<Tarefa>) -> impl Responder {
    match sqlx::query(
        "INSERT INTO tarefas (titulo, descricao, concluida) VALUES (?, ?, ?)",
    )
    .bind(&tarefa.titulo)
    .bind(&tarefa.descricao)
    .bind(tarefa.concluida)
    .execute(db.get_ref())
    .await
    {
        Ok(resultado) => {
            let id = resultado.last_insert_rowid();
            
            let response = ApiResponse {
                status: "sucesso".to_string(),
                data: format!("Tarefa criada com ID: {}", id),
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Erro ao criar tarefa: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Handler para atualizar uma tarefa existente
#[put("/tarefas/{id}")]
async fn atualizar_tarefa(
    db: web::Data<SqlitePool>,
    path: web::Path<i64>,
    tarefa: web::Json<Tarefa>,
) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query(
        "UPDATE tarefas SET titulo = ?, descricao = ?, concluida = ? WHERE id = ?",
    )
    .bind(&tarefa.titulo)
    .bind(&tarefa.descricao)
    .bind(tarefa.concluida)
    .bind(id)
    .execute(db.get_ref())
    .await
    {
        Ok(resultado) => {
            if resultado.rows_affected() > 0 {
                let response = ApiResponse {
                    status: "sucesso".to_string(),
                    data: format!("Tarefa atualizada com ID: {}", id),
                };
                HttpResponse::Ok().json(response)
            } else {
                let response = ApiResponse {
                    status: "erro".to_string(),
                    data: format!("Tarefa com ID {} não encontrada", id),
                };
                HttpResponse::NotFound().json(response)
            }
        }
        Err(e) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Erro ao atualizar tarefa: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Handler para excluir uma tarefa
#[delete("/tarefas/{id}")]
async fn excluir_tarefa(db: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    
    match sqlx::query("DELETE FROM tarefas WHERE id = ?")
        .bind(id)
        .execute(db.get_ref())
        .await
    {
        Ok(resultado) => {
            if resultado.rows_affected() > 0 {
                let response = ApiResponse {
                    status: "sucesso".to_string(),
                    data: format!("Tarefa excluída com ID: {}", id),
                };
                HttpResponse::Ok().json(response)
            } else {
                let response = ApiResponse {
                    status: "erro".to_string(),
                    data: format!("Tarefa com ID {} não encontrada", id),
                };
                HttpResponse::NotFound().json(response)
            }
        }
        Err(e) => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Erro ao excluir tarefa: {}", e),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Função para inicializar o banco de dados
async fn inicializar_banco(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tarefas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            descricao TEXT NOT NULL,
            concluida BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(pool)
    .await?;
    
    println!("Banco de dados inicializado com sucesso!");
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Carregar variáveis de ambiente do arquivo .env
    dotenv().ok();
    
    // Obter a URL do banco de dados do arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    
    // Conectar ao banco de dados SQLite
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");
    
    // Inicializar o banco de dados
    inicializar_banco(&pool)
        .await
        .expect("Falha ao inicializar o banco de dados");
    
    println!("Servidor iniciado em http://127.0.0.1:8080");
    
    // Iniciar o servidor
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(listar_tarefas)
            .service(obter_tarefa)
            .service(criar_tarefa)
            .service(atualizar_tarefa)
            .service(excluir_tarefa)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 