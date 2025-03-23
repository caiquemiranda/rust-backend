# Tutorial: Criando uma API REST com Rotas e Parâmetros em Rust

Neste tutorial, vamos criar uma API REST mais avançada usando Rust e Actix-web. Vamos explorar diferentes tipos de rotas, extração de parâmetros de URL e resposta em formato JSON.

## Passo 1: Configurando o Projeto

Primeiro, crie uma nova pasta para o projeto e inicialize um projeto Rust:

```bash
mkdir app02
cd app02
cargo init
```

## Passo 2: Configurando as Dependências

Edite o arquivo `Cargo.toml` para adicionar as dependências necessárias:

```toml
[package]
name = "app02"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Aqui estamos adicionando:
- `actix-web`: Framework web para Rust
- `serde`: Para serialização e deserialização de dados
- `serde_json`: Para trabalhar com JSON

## Passo 3: Definindo Estruturas de Dados

Vamos começar editando o arquivo `src/main.rs`. Primeiro, importamos as bibliotecas necessárias e definimos nossas estruturas de dados:

```rust
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Produto {
    id: u32,
    nome: String,
    preco: f32,
    disponivel: bool,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}
```

Neste código:
- `#[derive(Serialize, Deserialize)]` são macros do Serde para gerar automaticamente a implementação para serializar/deserializar a estrutura
- Criamos uma estrutura `Produto` para representar nossos produtos
- Criamos uma estrutura genérica `ApiResponse<T>` para padronizar nossas respostas da API

## Passo 4: Criando a Rota Principal

Agora, vamos criar nossa primeira rota:

```rust
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Produtos. Acesse /produtos para ver a lista completa.")
}
```

Esta é uma rota simples que retorna texto quando o usuário acessa a raiz da API.

## Passo 5: Criando uma Rota para Listar Produtos

Vamos adicionar uma rota para listar todos os produtos:

```rust
#[get("/produtos")]
async fn listar_produtos() -> impl Responder {
    let produtos = vec![
        Produto { id: 1, nome: "Celular".to_string(), preco: 1299.99, disponivel: true },
        Produto { id: 2, nome: "Notebook".to_string(), preco: 3999.99, disponivel: true },
        Produto { id: 3, nome: "Fone de Ouvido".to_string(), preco: 199.99, disponivel: false },
    ];

    let response = ApiResponse {
        status: "sucesso".to_string(),
        data: produtos,
    };

    HttpResponse::Ok().json(response)
}
```

Aqui:
- Criamos uma lista simulada de produtos
- Embalamos essa lista na nossa estrutura de resposta padronizada
- Usamos `.json()` para enviar a resposta como JSON

## Passo 6: Criando uma Rota com Parâmetros de Path

Agora, vamos criar uma rota que aceita um parâmetro de path (ID do produto):

```rust
#[get("/produtos/{id}")]
async fn obter_produto(path: web::Path<u32>) -> impl Responder {
    let produto_id = path.into_inner();
    
    // Base de dados simulada
    let produtos = vec![
        Produto { id: 1, nome: "Celular".to_string(), preco: 1299.99, disponivel: true },
        Produto { id: 2, nome: "Notebook".to_string(), preco: 3999.99, disponivel: true },
        Produto { id: 3, nome: "Fone de Ouvido".to_string(), preco: 199.99, disponivel: false },
    ];
    
    // Procura o produto pelo ID
    match produtos.iter().find(|p| p.id == produto_id) {
        Some(produto) => {
            let response = ApiResponse {
                status: "sucesso".to_string(),
                data: produto,
            };
            HttpResponse::Ok().json(response)
        },
        None => {
            let response = ApiResponse {
                status: "erro".to_string(),
                data: format!("Produto com ID {} não encontrado", produto_id),
            };
            HttpResponse::NotFound().json(response)
        }
    }
}
```

Neste endpoint:
- Usamos `web::Path<u32>` para extrair o ID do produto da URL
- Procuramos o produto com esse ID na nossa lista simulada
- Retornamos o produto se encontrado, ou uma mensagem de erro caso contrário

## Passo 7: Criando uma Rota com Parâmetros de Path mais Complexos

Vamos criar uma rota que mostra produtos por categoria:

```rust
#[get("/categorias/{categoria}/produtos")]
async fn produtos_por_categoria(path: web::Path<String>) -> impl Responder {
    let categoria = path.into_inner();
    
    let resposta = ApiResponse {
        status: "sucesso".to_string(),
        data: format!("Listando produtos da categoria: {}", categoria),
    };
    
    HttpResponse::Ok().json(resposta)
}
```

Esta rota demonstra como podemos ter rotas mais complexas com múltiplos segmentos.

## Passo 8: Criando uma Rota com Parâmetros de Query

Por fim, vamos criar uma rota que aceita parâmetros de query para busca:

```rust
#[get("/busca")]
async fn buscar_produtos(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let nome = query.get("nome").cloned().unwrap_or_else(|| "".to_string());
    let preco_max = query.get("preco_max").and_then(|p| p.parse::<f32>().ok()).unwrap_or(f32::MAX);
    
    let resposta = ApiResponse {
        status: "sucesso".to_string(),
        data: format!("Busca por produtos com nome '{}' e preço máximo {}", nome, preco_max),
    };
    
    HttpResponse::Ok().json(resposta)
}
```

Neste endpoint:
- Usamos `web::Query` para extrair os parâmetros de consulta da URL
- Processamos os parâmetros "nome" e "preco_max"
- Em uma aplicação real, usaríamos esses parâmetros para filtrar os produtos

## Passo 9: Configurando o Servidor

Finalmente, configuramos o servidor para usar todas as nossas rotas:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor iniciado em http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(listar_produtos)
            .service(obter_produto)
            .service(produtos_por_categoria)
            .service(buscar_produtos)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Passo 10: Executando e Testando

Para executar o servidor:

```bash
cargo run
```

Agora você pode testar os diferentes endpoints:

1. Página inicial:
```
curl http://localhost:8080/
```

2. Listar todos os produtos:
```
curl http://localhost:8080/produtos
```

3. Obter um produto específico pelo ID:
```
curl http://localhost:8080/produtos/1
```

4. Buscar produtos por categoria:
```
curl http://localhost:8080/categorias/eletronicos/produtos
```

5. Buscar produtos com parâmetros de consulta:
```
curl "http://localhost:8080/busca?nome=celular&preco_max=1500"
```

## Conclusão

Neste tutorial, você aprendeu a criar uma API REST mais avançada com Rust e Actix-web, incluindo:

- Como definir diferentes tipos de rotas
- Como extrair e processar parâmetros de path
- Como trabalhar com parâmetros de query
- Como estruturar e retornar respostas JSON

No próximo projeto, vamos avançar ainda mais e aprender como integrar um banco de dados SQLite para persistência de dados. 