.PHONY: all build test coverage bench clean doc install package

# Основная цель - собрать проект
all: build test

# Сборка проекта
build:
	cargo build

# Сборка в production режиме
release:
	cargo build --release

# Запуск тестов
test:
	cargo test

# Тесты с детализацией
test-verbose:
	cargo test -- --nocapture

# Генерация отчета о покрытии
coverage:
	cargo install cargo-tarpaulin --force
	cargo tarpaulin --ignore-tests --out Html

# Запуск бенчмарков
bench:
	cargo bench

# Очистка результатов сборки
clean:
	cargo clean

# Генерация документации
doc:
	cargo doc --no-deps

# Установка бинарного файла
install:
	cargo install --path .

# Создание установочного пакета
package:
	cargo install cargo-deb --force
	cargo deb

# Генерация примера лабиринта
generate:
	cargo run --bin generate_maze

# Запуск с примером лабиринта
run-example:
	cargo run -- --path examples/maze.txt
