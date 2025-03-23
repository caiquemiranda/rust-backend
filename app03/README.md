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

### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)
- SQLite 3 instalado no sistema

### Comandos
1. Entre no diretório do projeto:
```
cd app03
```

2. Execute o projeto:
```
cargo run
```

3. Teste os endpoints usando cURL ou outras ferramentas:

#### Listar todas as tarefas
```
curl http://localhost:8080/tarefas
```

#### Obter uma tarefa específica
```
curl http://localhost:8080/tarefas/1
```

#### Criar uma nova tarefa
```
curl -X POST http://localhost:8080/tarefas \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust","descricao":"Estudar APIs REST em Rust","concluida":false}'
```

#### Atualizar uma tarefa existente
```
curl -X PUT http://localhost:8080/tarefas/1 \
  -H "Content-Type: application/json" \
  -d '{"titulo":"Aprender Rust Avançado","descricao":"Implementar APIs REST em Rust","concluida":true}'
```

#### Excluir uma tarefa
```
curl -X DELETE http://localhost:8080/tarefas/1
```

O servidor estará disponível em `http://localhost:8080` 