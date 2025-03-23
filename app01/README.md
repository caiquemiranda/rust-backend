# App 01 - Hello World Server com Rust

## O que este projeto faz
Este é um servidor HTTP simples criado com Rust usando o framework Actix-Web. O servidor responde a requisições HTTP em dois endpoints:
- `/` - Retorna uma mensagem de boas-vindas
- `/sobre` - Retorna uma breve descrição do servidor

## O que este projeto ensina
- Configuração básica de um projeto Rust
- Instalação e uso de dependências externas com Cargo
- Criação de um servidor HTTP simples usando Actix-Web
- Definição de rotas e handlers básicos
- Como iniciar e executar um servidor web em Rust

## Como executar o projeto

### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

### Comandos
1. Entre no diretório do projeto:
```
cd app01
```

2. Execute o projeto:
```
cargo run
```

3. Acesse o servidor no navegador ou com ferramentas como cURL:
```
curl http://localhost:8080/
curl http://localhost:8080/sobre
```

O servidor estará disponível em `http://localhost:8080` 