# Tutorial: Criando uma API CRUD com SQLite em Rust

Neste tutorial, vamos criar uma API REST completa com operações CRUD (Create, Read, Update, Delete) integrada a um banco de dados SQLite usando Rust, Actix-web e SQLx.

## Passo 1: Configurando o Projeto

Primeiro, crie um novo diretório e inicialize um projeto Rust:

```bash
mkdir app03
cd app03
cargo init
```

## Passo 2: Configurando as Dependências

Edite o arquivo `Cargo.toml` para adicionar as dependências necessárias:

```toml
[package]
name = "app03"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "time", "macros"] }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
```

Aqui estamos adicionando:
- `actix-web`: Framework web para Rust
- `serde`: Para serialização e deserialização de dados
- `sqlx`: Para interação com o banco de dados SQLite
- `tokio`: Runtime assíncrono para Rust
- `dotenv`: Para carregamento de variáveis de ambiente

## Passo 3: Configurando a Conexão com o Banco de Dados

Crie um arquivo `.env` na raiz do projeto para armazenar a configuração da conexão com o banco de dados:

```
DATABASE_URL=sqlite:db.sqlite3
```

## Passo 4: Definindo a Estrutura de Dados

Agora, vamos editar o arquivo `src/main.rs`. Comece importando as bibliotecas necessárias e definindo a estrutura de dados para nossa aplicação:

```rust
use actix_web::{get, post, put, delete, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use dotenv::dotenv;
use std::env;

// Estrutura para representar uma tarefa
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
```

A estrutura `Tarefa` representa o modelo de dados da nossa aplicação, enquanto `ApiResponse<T>` é uma estrutura genérica para padronizar as respostas da API.

## Passo 5: Inicializando o Banco de Dados

Vamos adicionar uma função para inicializar o banco de dados:

```rust
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
```

Esta função cria a tabela `tarefas` se ela ainda não existir.

## Passo 6: Implementando os Endpoints da API

Agora, vamos implementar os endpoints da nossa API para operações CRUD:

### 6.1 Endpoint para a Página Inicial

```rust
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Tarefas com SQLite. Use /tarefas para gerenciar suas tarefas.")
}
```

### 6.2 Endpoint para Listar Todas as Tarefas (READ - All)

```rust
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
```

### 6.3 Endpoint para Obter Uma Tarefa por ID (READ - Single)

```rust
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
```

### 6.4 Endpoint para Criar uma Nova Tarefa (CREATE)

```rust
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
```

### 6.5 Endpoint para Atualizar uma Tarefa Existente (UPDATE)

```rust
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
```

### 6.6 Endpoint para Excluir uma Tarefa (DELETE)

```rust
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
```

## Passo 7: Configurando a Função Principal

Por fim, vamos implementar a função `main` para conectar ao banco de dados e iniciar o servidor:

```rust
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
```

## Passo 8: Executando e Testando

Para executar o servidor:

```bash
cargo run
```

Agora você pode testar os endpoints:

### Listar todas as tarefas
```bash
curl http://localhost:8080/tarefas
```

### Obter uma tarefa específica
```bash
curl http://localhost:8080/tarefas/1
```

### Criar uma nova tarefa
```bash
curl -X POST http://localhost:8080/tarefas \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust","descricao":"Estudar APIs REST em Rust","concluida":false}'
```

### Atualizar uma tarefa existente
```bash
curl -X PUT http://localhost:8080/tarefas/1 \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust Avançado","descricao":"Implementar APIs REST em Rust","concluida":true}'
```

### Excluir uma tarefa
```bash
curl -X DELETE http://localhost:8080/tarefas/1
```

## Conclusão

Parabéns! Você criou uma API REST completa com operações CRUD integrada a um banco de dados SQLite usando Rust. Este projeto demonstra:

1. Como integrar um banco de dados SQLite em uma aplicação Rust
2. Como implementar operações CRUD com SQLx
3. Como estruturar uma API REST mais complexa com Actix-web
4. Como tratar erros e validações em uma aplicação web com banco de dados

No próximo projeto, avançaremos ainda mais e implementaremos um sistema de comunicação em tempo real usando WebSockets. 