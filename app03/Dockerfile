# Estágio de compilação
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app

# Instalar dependências necessárias para compilação
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

# Estágio de produção
FROM debian:bullseye-slim

# Instalar dependências necessárias para execução
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copiar o binário compilado e arquivos necessários
COPY --from=builder /usr/src/app/target/release/app03 .
COPY .env .

# Expor a porta que o aplicativo utilizará
EXPOSE 8080

# Criar um volume para persistir o banco de dados
VOLUME ["/usr/local/bin/data"]

# Comando para executar o aplicativo
CMD ["./app03"] 