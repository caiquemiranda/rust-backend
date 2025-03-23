# Tutorial: Criando um Servidor HTTP Simples com Rust e Actix-web

Neste tutorial, vamos criar um servidor HTTP básico usando Rust e o framework Actix-web. Vamos aprender os fundamentos de como configurar um projeto Rust e criar um servidor web simples com rotas.

## Passo 1: Configurando o Ambiente

Primeiro, certifique-se de que você tem o Rust e o Cargo instalados. Se não tiver, instale através do site oficial https://www.rust-lang.org/tools/install.

Para verificar se a instalação foi bem-sucedida, execute:
```bash
rustc --version
cargo --version
```

## Passo 2: Criando um Novo Projeto Rust

Crie um novo diretório para o projeto e inicialize um novo projeto Rust:
```bash
mkdir app01
cd app01
cargo init
```

Isso criará a estrutura básica de um projeto Rust com um arquivo `Cargo.toml` e um diretório `src/` contendo `main.rs`.

## Passo 3: Adicionando a Dependência do Actix-web

Abra o arquivo `Cargo.toml` e adicione o Actix-web como uma dependência:
```toml
[package]
name = "app01"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4.0"
```

## Passo 4: Criando um Servidor HTTP Simples

Agora, vamos editar o arquivo `src/main.rs` para criar nosso servidor HTTP:
```rust
use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Olá, Mundo! Bem-vindo ao primeiro servidor Rust!")
}

#[get("/sobre")]
async fn sobre() -> impl Responder {
    HttpResponse::Ok().body("Este é um servidor HTTP simples criado com Rust e Actix-web.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor iniciado em http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(sobre)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Vamos entender o código:

1. Importamos as dependências necessárias do Actix-web.
2. Definimos duas funções de handler marcadas com `#[get("/")]` e `#[get("/sobre")]`, que respondem a requisições GET nos caminhos "/" e "/sobre".
3. Na função `main`, criamos um novo HttpServer que:
   - Configura uma nova aplicação com `App::new()`
   - Registra nossos serviços (handlers) na aplicação
   - Configura o servidor para escutar no endereço 127.0.0.1:8080
   - Inicia o servidor com `.run().await`

## Passo 5: Executando o Servidor

Para executar o servidor, use o comando:
```bash
cargo run
```

Este comando compilará o projeto e iniciará o servidor. Você verá a mensagem "Servidor iniciado em http://127.0.0.1:8080" no terminal.

## Passo 6: Testando o Servidor

Abra um navegador web e acesse http://127.0.0.1:8080/ - você deverá ver a mensagem "Olá, Mundo! Bem-vindo ao primeiro servidor Rust!".

Agora acesse http://127.0.0.1:8080/sobre - você deverá ver a mensagem "Este é um servidor HTTP simples criado com Rust e Actix-web.".

Alternativamente, você pode usar ferramentas de linha de comando como cURL:
```bash
curl http://127.0.0.1:8080/
curl http://127.0.0.1:8080/sobre
```

## Conclusão

Parabéns! Você criou seu primeiro servidor HTTP em Rust usando o framework Actix-web. Este é um exemplo simples, mas demonstra os conceitos fundamentais para criar aplicações web com Rust.

No próximo projeto, vamos expandir nosso conhecimento e criar uma API com múltiplas rotas e parâmetros dinâmicos. 