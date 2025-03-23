use actix_cors::Cors;
use actix_web::{
    delete, get, post, put, web, App, HttpResponse, HttpServer, Responder, Result,
};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use std::env;
use uuid::Uuid;

// Modelo de tarefa
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
struct Task {
    #[serde(default)]
    id: String,
    title: String,
    description: String,
    status: String,
    #[serde(default = "default_priority")]
    priority: i32,
    #[serde(default = "Utc::now")]
    created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    updated_at: DateTime<Utc>,
}

// Valor padrão para a prioridade
fn default_priority() -> i32 {
    1
}

// Modelo para atualização parcial de tarefas
#[derive(Debug, Deserialize)]
struct TaskUpdate {
    title: Option<String>,
    description: Option<String>,
    status: Option<String>,
    priority: Option<i32>,
}

// Modelo de resposta da API
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

// Handler para a rota raiz
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "API de Gerenciamento de Tarefas".to_string(),
        data: None::<()>,
    })
}

// Handler para listar todas as tarefas
#[get("/tasks")]
async fn get_tasks(db: web::Data<SqlitePool>) -> Result<impl Responder> {
    match sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(db.get_ref())
        .await
    {
        Ok(tasks) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Tarefas recuperadas com sucesso".to_string(),
            data: Some(tasks),
        })),
        Err(e) => {
            log::error!("Erro ao listar tarefas: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: format!("Erro ao recuperar tarefas: {}", e),
                data: None,
            }))
        }
    }
}

// Handler para obter uma tarefa específica pelo ID
#[get("/tasks/{id}")]
async fn get_task(db: web::Data<SqlitePool>, path: web::Path<String>) -> Result<impl Responder> {
    let id = path.into_inner();

    match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(Some(task)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Tarefa recuperada com sucesso".to_string(),
            data: Some(task),
        })),
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            message: format!("Tarefa com ID {} não encontrada", id),
            data: None,
        })),
        Err(e) => {
            log::error!("Erro ao buscar tarefa: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: format!("Erro ao buscar tarefa: {}", e),
                data: None,
            }))
        }
    }
}

// Handler para criar uma nova tarefa
#[post("/tasks")]
async fn create_task(
    db: web::Data<SqlitePool>,
    task: web::Json<Task>,
) -> Result<impl Responder> {
    let mut new_task = task.into_inner();
    new_task.id = Uuid::new_v4().to_string();
    new_task.created_at = Utc::now();
    new_task.updated_at = Utc::now();

    match sqlx::query(
        r#"
        INSERT INTO tasks (id, title, description, status, priority, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&new_task.id)
    .bind(&new_task.title)
    .bind(&new_task.description)
    .bind(&new_task.status)
    .bind(new_task.priority)
    .bind(new_task.created_at)
    .bind(new_task.updated_at)
    .execute(db.get_ref())
    .await
    {
        Ok(_) => Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            message: "Tarefa criada com sucesso".to_string(),
            data: Some(new_task),
        })),
        Err(e) => {
            log::error!("Erro ao criar tarefa: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: format!("Erro ao criar tarefa: {}", e),
                data: None,
            }))
        }
    }
}

// Handler para atualizar uma tarefa existente
#[put("/tasks/{id}")]
async fn update_task(
    db: web::Data<SqlitePool>,
    path: web::Path<String>,
    update: web::Json<TaskUpdate>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let update = update.into_inner();
    let now = Utc::now();

    // Primeiro, verifica se a tarefa existe
    match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_optional(db.get_ref())
        .await
    {
        Ok(Some(existing_task)) => {
            // Atualiza apenas os campos que foram fornecidos
            let title = update.title.unwrap_or(existing_task.title);
            let description = update.description.unwrap_or(existing_task.description);
            let status = update.status.unwrap_or(existing_task.status);
            let priority = update.priority.unwrap_or(existing_task.priority);

            match sqlx::query(
                r#"
                UPDATE tasks 
                SET title = ?, description = ?, status = ?, priority = ?, updated_at = ?
                WHERE id = ?
                "#,
            )
            .bind(&title)
            .bind(&description)
            .bind(&status)
            .bind(priority)
            .bind(now)
            .bind(&id)
            .execute(db.get_ref())
            .await
            {
                Ok(_) => {
                    // Retorna a tarefa atualizada
                    let updated_task = Task {
                        id: existing_task.id,
                        title,
                        description,
                        status,
                        priority,
                        created_at: existing_task.created_at,
                        updated_at: now,
                    };

                    Ok(HttpResponse::Ok().json(ApiResponse {
                        success: true,
                        message: "Tarefa atualizada com sucesso".to_string(),
                        data: Some(updated_task),
                    }))
                }
                Err(e) => {
                    log::error!("Erro ao atualizar tarefa: {}", e);
                    Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        success: false,
                        message: format!("Erro ao atualizar tarefa: {}", e),
                        data: None,
                    }))
                }
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            message: format!("Tarefa com ID {} não encontrada", id),
            data: None,
        })),
        Err(e) => {
            log::error!("Erro ao buscar tarefa: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: format!("Erro ao buscar tarefa: {}", e),
                data: None,
            }))
        }
    }
}

// Handler para excluir uma tarefa
#[delete("/tasks/{id}")]
async fn delete_task(
    db: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    match sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(&id)
        .execute(db.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(HttpResponse::Ok().json(ApiResponse::<()> {
                    success: true,
                    message: format!("Tarefa com ID {} excluída com sucesso", id),
                    data: None,
                }))
            } else {
                Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
                    success: false,
                    message: format!("Tarefa com ID {} não encontrada", id),
                    data: None,
                }))
            }
        }
        Err(e) => {
            log::error!("Erro ao excluir tarefa: {}", e);
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: format!("Erro ao excluir tarefa: {}", e),
                data: None,
            }))
        }
    }
}

// Inicializa o banco de dados
async fn init_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Cria a tabela de tarefas se ela não existir
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            priority INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    log::info!("Banco de dados inicializado com sucesso");
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Obtém a configuração do arquivo .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT deve ser um número");

    // Conecta ao banco de dados SQLite
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    // Inicializa o banco de dados
    init_database(&pool)
        .await
        .expect("Falha ao inicializar o banco de dados");

    log::info!("Servidor iniciado em http://127.0.0.1:{}", server_port);

    // Inicia o servidor HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(index)
            .service(get_tasks)
            .service(get_task)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind(("127.0.0.1", server_port))?
    .run()
    .await
} 