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

### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

### Comandos
1. Entre no diretório do projeto:
```
cd app04
```

2. Execute o projeto:
```
cargo run
```

3. Abra o navegador e acesse:
```
http://localhost:8080
```

4. Para testar o chat com múltiplos usuários:
   - Abra várias janelas do navegador apontando para http://localhost:8080
   - Defina nomes de usuários diferentes em cada janela
   - Envie mensagens e observe a comunicação em tempo real entre as janelas

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

O servidor estará disponível em `http://localhost:8080` 