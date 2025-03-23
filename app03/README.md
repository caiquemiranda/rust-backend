# App 03 - API CRUD com SQLite em Rust

## O que este projeto faz
Este projeto implementa uma API REST completa com operações CRUD (Create, Read, Update, Delete) para gerenciar tarefas, armazenando os dados em um banco de dados SQLite. A API inclui os seguintes endpoints:

- `GET /` - Página inicial com instruções
- `GET /tarefas` - Lista todas as tarefas
- `GET /tarefas/{id}` - Obtém uma tarefa específica pelo ID
- `POST /tarefas` - Cria uma nova tarefa
- `PUT /tarefas/{id}` - Atualiza uma tarefa existente
- `DELETE /tarefas/{id}` - Remove uma tarefa

## O que este projeto ensina
- Integração de um banco de dados SQLite com Rust usando SQLx
- Operações CRUD completas (Create, Read, Update, Delete)
- Manipulação de erros em um contexto de banco de dados
- Uso de migrações e inicialização de banco de dados
- Configuração de ambiente com variáveis de ambiente usando dotenv
- Serialização e deserialização com mapeamento de banco de dados
- Estruturação de um projeto web mais completo em Rust

## Como executar o projeto

### Método 1: Usando Rust diretamente

#### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)
- SQLite 3 instalado no sistema

#### Comandos
1. Entre no diretório do projeto:
```
cd app03
```

2. Configure o arquivo `.env` na raiz do projeto:
```
DATABASE_URL=sqlite:db.sqlite3
```

3. Execute o projeto:
```
cargo run
```

### Método 2: Usando Docker

#### Pré-requisitos
- Docker e Docker Compose instalados (https://docs.docker.com/get-docker/)

#### Comandos
1. Entre no diretório do projeto:
```
cd app03
```

2. Construa e inicie o contêiner:
```
docker-compose up
```

3. Para executar em segundo plano:
```
docker-compose up -d
```

4. Para parar o contêiner (os dados do banco persistirão no volume):
```
docker-compose down
```

5. Para remover o contêiner e os dados do volume:
```
docker-compose down -v
```

## Testando o projeto

O servidor estará disponível em `http://localhost:8080`. Você pode testar os endpoints usando cURL ou outras ferramentas:

### Listar todas as tarefas
```
curl http://localhost:8080/tarefas
```

### Obter uma tarefa específica
```
curl http://localhost:8080/tarefas/1
```

### Criar uma nova tarefa
```
curl -X POST http://localhost:8080/tarefas \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust","descricao":"Estudar APIs REST em Rust","concluida":false}'
```

### Atualizar uma tarefa existente
```
curl -X PUT http://localhost:8080/tarefas/1 \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust e SQLite","concluida":true}'
```

### Excluir uma tarefa
```
curl -X DELETE http://localhost:8080/tarefas/1
``` 