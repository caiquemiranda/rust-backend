# Tutoriais de Rust Backend

Este repositório contém uma série de tutoriais para aprender desenvolvimento de backend com Rust, começando do básico e progredindo para aplicações mais complexas.

## Projetos Disponíveis

### 1. App01 - Hello World com Actix Web
Uma introdução simples ao framework Actix Web, criando um servidor HTTP básico que responde com "Hello World".

**Conceitos abordados:**
- Configuração inicial do projeto Rust
- Uso básico do Actix Web
- Criação de rotas simples
- Compilação e execução de uma aplicação web Rust

### 2. App02 - API REST Básica
Uma API REST básica que demonstra os princípios fundamentais de construção de APIs com Rust.

**Conceitos abordados:**
- Criação de múltiplas rotas e handlers
- Manipulação de diferentes métodos HTTP (GET, POST, PUT, DELETE)
- Serialização e desserialização com Serde
- Gerenciamento básico de estado em memória

### 3. App03 - API REST com Banco de Dados
Uma API REST conectada a um banco de dados SQLite, demonstrando como persistir dados.

**Conceitos abordados:**
- Integração com banco de dados SQLite usando SQLx
- Operações CRUD completas
- Migrations de banco de dados
- Tratamento de erros mais robusto
- Estruturação de projetos maiores

### 4. App04 - Chat em Tempo Real com WebSockets
Um sistema de chat em tempo real implementado com WebSockets, permitindo comunicação bidirecional.

**Conceitos abordados:**
- Implementação de WebSockets com Actix
- Comunicação em tempo real entre cliente e servidor
- Gerenciamento de sessões de usuário
- Broadcasts e mensagens direcionadas
- Interface de usuário com HTML, CSS e JavaScript

### 5. App05 - Aplicação CRUD Completa com Frontend React
Uma aplicação CRUD completa com backend Rust e frontend React, demonstrando uma arquitetura full-stack.

**Conceitos abordados:**
- Construção de API RESTful com Actix Web
- Modelagem de dados com SQLx e SQLite
- CORS e integração com frontend
- Frontend React com Material UI
- Gerenciamento de estado com Context API
- TypeScript para tipagem segura

## Como Usar Este Repositório

Cada diretório de projeto contém:
- Código-fonte completo
- Um arquivo `tutorial.md` detalhado que guia você pelo processo de criação do projeto
- Um arquivo `Cargo.toml` com as dependências necessárias
- Arquivos `Dockerfile` e `docker-compose.yml` para execução com Docker

## Requisitos

- Rust (versão 1.65.0 ou superior)
- Cargo (instalado com Rust)
- SQLite (para os projetos com banco de dados)
- Node.js e npm (para o projeto com React)
- Docker e Docker Compose (para execução com contêineres)

## Executando os Projetos

### Usando Rust diretamente

1. Clone este repositório:
   ```
   git clone https://github.com/caiquemiranda/rust-backend-tutorials.git
   ```

2. Navegue até a pasta do projeto desejado:
   ```
   cd rust-backend-tutorials/app01
   ```

3. Execute o projeto:
   ```
   cargo run
   ```

### Usando Docker

Cada projeto possui uma configuração Docker pronta para uso.

1. Clone este repositório:
   ```
   git clone https://github.com/caiquemiranda/rust-backend-tutorials.git
   ```

2. Navegue até a pasta do projeto desejado:
   ```
   cd rust-backend-tutorials/app01
   ```

3. Execute o projeto usando Docker Compose:
   ```
   docker-compose up
   ```

4. Para executar em segundo plano:
   ```
   docker-compose up -d
   ```

5. Para parar o contêiner:
   ```
   docker-compose down
   ```

### Detalhes de Portas e URLs

- Todos os aplicativos são expostos na porta **8080**
- Acesse os projetos em: **http://localhost:8080**

## Contribuindo

Contribuições são bem-vindas! Se você encontrar um problema ou quiser melhorar os tutoriais, sinta-se à vontade para abrir uma issue ou enviar um pull request.

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo LICENSE para mais detalhes.