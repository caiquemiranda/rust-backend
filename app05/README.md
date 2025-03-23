# API de Gerenciamento de Tarefas com React

Este projeto é uma aplicação completa de gerenciamento de tarefas (CRUD) com backend em Rust e frontend em React.

## Backend (Rust)

O backend é uma API RESTful construída com:

- **Actix-web**: Framework web rápido e pragmático
- **SQLx**: Biblioteca SQL assíncrona para Rust
- **SQLite**: Banco de dados leve e embutido
- **Serde**: Serialização/deserialização de dados
- **UUID**: Geração de identificadores únicos
- **Chrono**: Manipulação de datas e horas
- **CORS**: Suporte para compartilhamento de recursos entre origens diferentes

### Funcionalidades

- Listagem de todas as tarefas
- Busca de tarefa por ID
- Criação de novas tarefas
- Atualização de tarefas existentes
- Exclusão de tarefas

### Modelo de Dados

```rust
struct Task {
    id: String,
    title: String,
    description: String,
    status: String,
    priority: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### Endpoints da API

| Método | Endpoint     | Descrição                   |
|--------|--------------|----------------------------|
| GET    | /            | Rota raiz/informativa      |
| GET    | /tasks       | Listar todas as tarefas    |
| GET    | /tasks/{id}  | Buscar tarefa por ID       |
| POST   | /tasks       | Criar nova tarefa          |
| PUT    | /tasks/{id}  | Atualizar tarefa existente |
| DELETE | /tasks/{id}  | Excluir tarefa             |

## Frontend (React)

O frontend é uma aplicação React moderna com:

- **React**: Biblioteca JavaScript para construção de interfaces
- **TypeScript**: Tipagem estática para JavaScript
- **Material-UI**: Biblioteca de componentes UI
- **React Router**: Navegação entre páginas
- **Axios**: Cliente HTTP para comunicação com a API

### Funcionalidades do Frontend

- Listagem de tarefas em uma interface amigável
- Formulário para criação de novas tarefas
- Edição de tarefas existentes
- Exclusão de tarefas
- Filtragem e ordenação de tarefas
- Interface responsiva

## Como Executar

### Backend

1. Certifique-se de ter o Rust e o Cargo instalados
2. Configure o arquivo `.env` na raiz do projeto:
   ```
   DATABASE_URL=sqlite:db.sqlite3
   SERVER_PORT=8080
   ```
3. Execute o servidor:
   ```bash
   cd app05
   cargo run
   ```
4. O servidor estará disponível em `http://localhost:8080`

### Frontend

1. Certifique-se de ter o Node.js e npm instalados
2. Navegue até a pasta do frontend:
   ```bash
   cd app05/frontend
   ```
3. Instale as dependências:
   ```bash
   npm install
   ```
4. Inicie a aplicação:
   ```bash
   npm start
   ```
5. O frontend estará disponível em `http://localhost:3000`

## Estrutura do Projeto

```
app05/
├── src/              # Código fonte do backend
│   └── main.rs       # Ponto de entrada do backend
├── frontend/         # Aplicação React
│   ├── public/       # Arquivos públicos
│   └── src/          # Código fonte do frontend
├── Cargo.toml        # Configuração do projeto Rust
├── .env              # Variáveis de ambiente
└── README.md         # Este arquivo
```

## Próximos Passos

- Adicionar autenticação de usuários
- Implementar categorias para tarefas
- Criar testes automatizados
- Adicionar funcionalidade de upload de arquivos
- Implementar notificações em tempo real 