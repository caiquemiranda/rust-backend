# Tutorial: Criando uma API CRUD Completa com Rust e React

Neste tutorial, criaremos uma aplicação completa de gerenciamento de tarefas com um backend Rust usando Actix-web e SQLite, e um frontend moderno em React com TypeScript e Material UI.

## Parte 1: Configurando o Projeto Backend

### Passo 1: Criando a Estrutura do Projeto

Primeiro, vamos criar um novo diretório para o projeto e inicializar um projeto Rust:

```bash
mkdir app05
cd app05
cargo init
```

### Passo 2: Configurando as Dependências

Edite o arquivo `Cargo.toml` para adicionar as dependências necessárias:

```toml
[package]
name = "app05"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4.0"
actix-cors = "0.6.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "time", "macros"] }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
uuid = { version = "1.4", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
env_logger = "0.10.0"
log = "0.4"
```

### Passo 3: Configurando o Ambiente

Crie um arquivo `.env` na raiz do projeto para armazenar as variáveis de ambiente:

```
DATABASE_URL=sqlite:db.sqlite3
SERVER_PORT=8080
```

### Passo 4: Implementando o Backend

Agora vamos criar o arquivo `src/main.rs` com nosso código principal:

```rust
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
```

### Passo 5: Testando o Backend

Inicie o servidor e teste os endpoints da API:

```bash
cd app05
cargo run
```

Você pode usar ferramentas como cURL, Postman ou Insomnia para testar os endpoints da API:

```bash
# Obter informações da API
curl http://localhost:8080/

# Listar tarefas
curl http://localhost:8080/tasks

# Criar uma nova tarefa
curl -X POST http://localhost:8080/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Aprender Rust","description":"Estudar Actix e React","status":"Pendente","priority":2}'

# Obter uma tarefa específica (substitua ID_DA_TAREFA pelo ID real)
curl http://localhost:8080/tasks/ID_DA_TAREFA

# Atualizar uma tarefa (substitua ID_DA_TAREFA pelo ID real)
curl -X PUT http://localhost:8080/tasks/ID_DA_TAREFA \
  -H "Content-Type: application/json" \
  -d '{"status":"Em Andamento"}'

# Excluir uma tarefa (substitua ID_DA_TAREFA pelo ID real)
curl -X DELETE http://localhost:8080/tasks/ID_DA_TAREFA
```

## Parte 2: Criando o Frontend com React

Agora vamos criar o frontend da nossa aplicação usando React com TypeScript e Material UI.

### Passo 1: Configurando o Projeto React

Primeiro, vamos criar a estrutura do projeto React dentro da pasta do nosso projeto Rust:

```bash
mkdir -p app05/frontend/src app05/frontend/public
cd app05/frontend
```

### Passo 2: Configurando o package.json

Crie um arquivo `package.json` na pasta frontend:

```json
{
  "name": "task-manager-frontend",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "@emotion/react": "^11.10.6",
    "@emotion/styled": "^11.10.6",
    "@mui/icons-material": "^5.11.16",
    "@mui/material": "^5.12.0",
    "@testing-library/jest-dom": "^5.16.5",
    "@testing-library/react": "^13.4.0",
    "@testing-library/user-event": "^13.5.0",
    "@types/jest": "^27.5.2",
    "@types/node": "^16.18.23",
    "@types/react": "^18.0.35",
    "@types/react-dom": "^18.0.11",
    "axios": "^1.3.5",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.10.0",
    "react-scripts": "5.0.1",
    "typescript": "^4.9.5",
    "web-vitals": "^2.1.4"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app",
      "react-app/jest"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  },
  "proxy": "http://localhost:8080"
}
```

O campo `"proxy"` no final permite que o servidor de desenvolvimento do React encaminhe as requisições para o nosso backend Rust.

### Passo 3: Configurando o TypeScript

Crie um arquivo `tsconfig.json` na pasta frontend:

```json
{
  "compilerOptions": {
    "target": "es5",
    "lib": [
      "dom",
      "dom.iterable",
      "esnext"
    ],
    "allowJs": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": [
    "src"
  ]
}
```

Também crie um arquivo `src/react-app-env.d.ts`:

```typescript
/// <reference types="react-scripts" />
```

### Passo 4: Configurando os Arquivos HTML

Crie um arquivo `public/index.html`:

```html
<!DOCTYPE html>
<html lang="pt-BR">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%PUBLIC_URL%/favicon.ico" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#000000" />
    <meta
      name="description"
      content="Aplicação de gerenciamento de tarefas criada com React e Rust"
    />
    <link rel="apple-touch-icon" href="%PUBLIC_URL%/logo192.png" />
    <link rel="manifest" href="%PUBLIC_URL%/manifest.json" />
    <link
      rel="stylesheet"
      href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
    />
    <title>Gerenciador de Tarefas</title>
  </head>
  <body>
    <noscript>Você precisa habilitar JavaScript para executar este aplicativo.</noscript>
    <div id="root"></div>
  </body>
</html>
```

E um arquivo `public/manifest.json`:

```json
{
  "short_name": "Gerenciador de Tarefas",
  "name": "Aplicação de Gerenciamento de Tarefas",
  "icons": [
    {
      "src": "favicon.ico",
      "sizes": "64x64 32x32 24x24 16x16",
      "type": "image/x-icon"
    }
  ],
  "start_url": ".",
  "display": "standalone",
  "theme_color": "#000000",
  "background_color": "#ffffff"
}
```

### Passo 5: Configurando os Tipos

Crie o arquivo `src/types/Task.ts` para definir os tipos para nossa aplicação:

```typescript
export interface Task {
  id: string;
  title: string;
  description: string;
  status: string;
  priority: number;
  created_at: string;
  updated_at: string;
}

export type TaskFormData = Omit<Task, 'id' | 'created_at' | 'updated_at'>;

export type TaskUpdateData = Partial<TaskFormData>;

export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
}

export enum TaskStatus {
  TODO = 'Pendente',
  IN_PROGRESS = 'Em Andamento',
  DONE = 'Concluída',
  CANCELLED = 'Cancelada',
}

export enum TaskPriority {
  LOW = 1,
  MEDIUM = 2,
  HIGH = 3,
  URGENT = 4,
}

export const TaskPriorityLabels: Record<TaskPriority, string> = {
  [TaskPriority.LOW]: 'Baixa',
  [TaskPriority.MEDIUM]: 'Média',
  [TaskPriority.HIGH]: 'Alta',
  [TaskPriority.URGENT]: 'Urgente',
};

export const TaskPriorityColors: Record<TaskPriority, string> = {
  [TaskPriority.LOW]: '#58b09c',
  [TaskPriority.MEDIUM]: '#f9c74f',
  [TaskPriority.HIGH]: '#f8961e',
  [TaskPriority.URGENT]: '#f25c54',
};

export const TaskStatusColors: Record<string, string> = {
  [TaskStatus.TODO]: '#1976d2',
  [TaskStatus.IN_PROGRESS]: '#9c27b0',
  [TaskStatus.DONE]: '#388e3c',
  [TaskStatus.CANCELLED]: '#757575',
};
```

### Passo 6: Configurando o Serviço de API

Crie o arquivo `src/services/api.ts` para comunicação com o backend:

```typescript
import axios from 'axios';
import { Task, TaskFormData, TaskUpdateData, ApiResponse } from '../types/Task';

const api = axios.create({
  baseURL: '/',
  headers: {
    'Content-Type': 'application/json',
  },
});

export const getTasks = async (): Promise<Task[]> => {
  try {
    const response = await api.get<ApiResponse<Task[]>>('/tasks');
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    return [];
  } catch (error) {
    console.error('Erro ao buscar tarefas:', error);
    return [];
  }
};

export const getTask = async (id: string): Promise<Task | null> => {
  try {
    const response = await api.get<ApiResponse<Task>>(`/tasks/${id}`);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    return null;
  } catch (error) {
    console.error(`Erro ao buscar tarefa ${id}:`, error);
    return null;
  }
};

export const createTask = async (taskData: TaskFormData): Promise<Task | null> => {
  try {
    const response = await api.post<ApiResponse<Task>>('/tasks', taskData);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    return null;
  } catch (error) {
    console.error('Erro ao criar tarefa:', error);
    return null;
  }
};

export const updateTask = async (id: string, taskData: TaskUpdateData): Promise<Task | null> => {
  try {
    const response = await api.put<ApiResponse<Task>>(`/tasks/${id}`, taskData);
    if (response.data.success && response.data.data) {
      return response.data.data;
    }
    return null;
  } catch (error) {
    console.error(`Erro ao atualizar tarefa ${id}:`, error);
    return null;
  }
};

export const deleteTask = async (id: string): Promise<boolean> => {
  try {
    const response = await api.delete<ApiResponse<null>>(`/tasks/${id}`);
    return response.data.success;
  } catch (error) {
    console.error(`Erro ao excluir tarefa ${id}:`, error);
    return false;
  }
};

export default api;
``` 