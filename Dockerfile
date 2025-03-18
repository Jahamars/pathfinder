# Используем официальный образ Rust
FROM rust:latest

# Устанавливаем рабочую директорию внутри контейнера
WORKDIR /app

# Копируем все файлы проекта в контейнер
COPY . .

# Компилируем проект в режиме release
RUN cargo build --release

# Указываем команду для запуска программы
CMD ["./target/release/pathfinder"]
