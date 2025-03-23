# Estágio de compilação
FROM rust:1.75-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Estágio de produção
FROM debian:bullseye-slim

# Instalar dependências necessárias
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copiar o binário compilado do estágio de compilação
COPY --from=builder /usr/src/app/target/release/app01 .

# Expor a porta que o aplicativo utilizará
EXPOSE 8080

# Comando para executar o aplicativo
CMD ["./app01"] 