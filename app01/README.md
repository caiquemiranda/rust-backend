# App 01 - Hello World com Actix Web

## O que este projeto faz
Este projeto implementa um servidor HTTP simples usando Rust e o framework Actix-web. Ele responde a requisições GET em dois endpoints:

- `/` - Retorna uma mensagem de boas-vindas
- `/sobre` - Retorna uma breve descrição do servidor

## O que este projeto ensina
- Como configurar um projeto Rust básico
- Como adicionar dependências ao Cargo.toml
- Como criar um servidor HTTP simples com Actix-web
- Como definir rotas e handlers para responder a requisições HTTP
- Como iniciar o servidor HTTP e vinculá-lo a um endereço

## Como executar o projeto

### Método 1: Usando Rust diretamente

#### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

#### Comandos
1. Entre no diretório do projeto:
```
cd app01
```

2. Execute o projeto:
```
cargo run
```

### Método 2: Usando Docker

#### Pré-requisitos
- Docker e Docker Compose instalados (https://docs.docker.com/get-docker/)

#### Comandos
1. Entre no diretório do projeto:
```
cd app01
```

2. Construa e inicie o contêiner:
```
docker-compose up
```

3. Para executar em segundo plano:
```
docker-compose up -d
```

4. Para parar o contêiner:
```
docker-compose down
```

## Testando o projeto

O servidor estará disponível em `http://localhost:8080`. Você pode acessar:

- `http://localhost:8080/` - Para ver a mensagem de boas-vindas
- `http://localhost:8080/sobre` - Para ver a descrição do servidor 