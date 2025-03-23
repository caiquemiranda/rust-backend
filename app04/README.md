# App 04 - Chat em Tempo Real com WebSockets em Rust

## O que este projeto faz
Este projeto implementa um sistema de chat em tempo real usando WebSockets com Rust no backend e JavaScript no frontend. O sistema permite que múltiplos usuários se conectem, enviem mensagens e vejam atualizações em tempo real. Funcionalidades incluem:

- Conexão em tempo real usando WebSockets
- Interface de chat amigável com HTML/CSS/JavaScript puro
- Escolha e alteração de nomes de usuários
- Mensagens do sistema para eventos (entrada/saída de usuários, mudança de nome)
- Histórico de mensagens preservado para novos usuários
- Reconexão automática em caso de desconexão

## O que este projeto ensina
- Implementação de WebSockets em Rust usando o framework Actix
- Comunicação em tempo real e bidirecional entre servidor e clientes
- Gerenciamento de estado e sessões usando o modelo Actor
- Transmissão de mensagens para múltiplos clientes (broadcast)
- Serialização e deserialização de mensagens JSON
- Manipulação de eventos do lado do cliente com JavaScript
- Criação de interfaces interativas para aplicações em tempo real

## Como executar o projeto

### Método 1: Usando Rust diretamente

#### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

#### Comandos
1. Entre no diretório do projeto:
```
cd app04
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
cd app04
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

O servidor estará disponível em `http://localhost:8080`. Para testar o chat:

1. Abra o navegador e acesse `http://localhost:8080`
2. Digite um nome de usuário e entre no chat
3. Para testar com múltiplos usuários, abra outras abas ou navegadores e conecte com nomes diferentes
4. Envie mensagens entre as janelas para ver a comunicação em tempo real

### Estrutura de mensagens WebSocket

As mensagens trocadas entre cliente e servidor seguem este formato JSON:
```json
// Cliente para servidor
{
  "action": "message" | "setUsername",
  "message": "texto da mensagem",
  "username": "nome do usuário"
}

// Servidor para cliente
{
  "id": "uuid-único",
  "username": "nome do usuário",
  "message": "texto da mensagem",
  "timestamp": 1234567890
}
``` 