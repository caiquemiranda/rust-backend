# Estágio de compilação do backend
FROM rust:1.75-slim as backend-builder

WORKDIR /usr/src/app

# Instalar dependências necessárias para compilação
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Estágio de compilação do frontend
FROM node:20-slim as frontend-builder

WORKDIR /usr/src/app/frontend

# Copiar arquivos de configuração do frontend
COPY frontend/package*.json ./
RUN npm ci

# Copiar o código fonte do frontend
COPY frontend/ ./

# Construir frontend
RUN npm run build

# Estágio de produção
FROM debian:bullseye-slim

# Instalar dependências necessárias para execução
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copiar o binário compilado e arquivos necessários
COPY --from=backend-builder /usr/src/app/target/release/app05 .
COPY .env .

# Criar diretório para os arquivos estáticos do frontend
RUN mkdir -p /usr/local/bin/static

# Copiar o frontend compilado para o diretório de arquivos estáticos
COPY --from=frontend-builder /usr/src/app/frontend/build /usr/local/bin/static

# Criar um volume para persistir o banco de dados
VOLUME ["/usr/local/bin/data"]

# Expor a porta que o aplicativo utilizará
EXPOSE 8080

# Comando para executar o aplicativo
CMD ["./app05"] 