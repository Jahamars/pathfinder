FROM rust:1.83-slim as builder

WORKDIR /app

# Копируем только файлы, нужные для зависимостей
COPY Cargo.toml Cargo.lock ./

# Создаем фиктивный main.rs для кэширования зависимостей
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/pathfinder*

# Копируем исходный код
COPY . .

# Собираем приложение
RUN cargo build --release

# Многоступенчатая сборка для уменьшения размера образа
FROM debian:bullseye-slim

# Установка только необходимых зависимостей
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    update-ca-certificates

WORKDIR /app

# Копирование бинарного файла из предыдущего этапа
COPY --from=builder /app/target/release/pathfinder .

# Создание непривилегированного пользователя
RUN useradd -m appuser
USER appuser

# Определение точки входа
ENTRYPOINT ["./pathfinder"]
CMD ["--path", "/app/maze.txt"]
