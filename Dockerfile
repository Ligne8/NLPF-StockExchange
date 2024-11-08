# Étape 1 : Construction de l'application
FROM rust:1.82 AS builder

# Définir le répertoire de travail
WORKDIR /app

# Copier les fichiers de configuration
COPY Cargo.toml Cargo.lock ./

# Créer un projet vide pour mettre en cache les dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compiler les dépendances
RUN cargo build --release && rm -rf src

# Copier le code source
COPY src ./src

# Compiler le projet pour la release
RUN cargo build --release

# Étape 2 : Image minimale pour exécuter l'application
FROM debian:bookworm-slim

# Copier le binaire compilé depuis l'étape de construction
COPY --from=builder /app/target/release/nlpf-StockExchange /usr/local/bin/nlpf-StockExchange

EXPOSE 5151

# Démarrer l'application
CMD ["nlpf-StockExchange"]