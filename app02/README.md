# App 02 - Rotas e Parâmetros em Rust

## O que este projeto faz
Este projeto implementa uma API REST que demonstra o uso de vários tipos de rotas e parâmetros em um servidor Rust. A API simula um sistema de produtos com os seguintes endpoints:

- `/` - Página inicial com instruções
- `/produtos` - Lista todos os produtos
- `/produtos/{id}` - Obtém detalhes de um produto específico pelo ID
- `/categorias/{categoria}/produtos` - Lista produtos por categoria
- `/busca?nome=X&preco_max=Y` - Busca produtos por nome e preço máximo

## O que este projeto ensina
- Criação de APIs RESTful com Rust
- Definição de múltiplas rotas com diferentes padrões
- Extração de parâmetros de rota (path parameters)
- Extração de parâmetros de consulta (query parameters)
- Serialização e deserialização de dados JSON usando Serde
- Estruturação e organização de respostas de API

## Como executar o projeto

### Pré-requisitos
- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

### Comandos
1. Entre no diretório do projeto:
```
cd app02
```

2. Execute o projeto:
```
cargo run
```

3. Teste os endpoints usando cURL ou um navegador:
```
curl http://localhost:8080/
curl http://localhost:8080/produtos
curl http://localhost:8080/produtos/1
curl http://localhost:8080/categorias/eletronicos/produtos
curl http://localhost:8080/busca?nome=celular&preco_max=1500
```

O servidor estará disponível em `http://localhost:8080` 