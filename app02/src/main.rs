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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("API de Produtos. Acesse /produtos para ver a lista completa.")
}

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

#[get("/categorias/{categoria}/produtos")]
async fn produtos_por_categoria(path: web::Path<String>) -> impl Responder {
    let categoria = path.into_inner();
    
    let resposta = ApiResponse {
        status: "sucesso".to_string(),
        data: format!("Listando produtos da categoria: {}", categoria),
    };
    
    HttpResponse::Ok().json(resposta)
}

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