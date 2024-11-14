# Компіляція та запуск програми в режимі розробки
run:
	cargo run

# Компіляція та запуск програми в режимі релізу
release:
	cargo run --release

# Збірка в режимі розробки
build:
	cargo build

# Збірка в режимі релізу
build-release:
	cargo build --release

# Запуск тестів
test:
	cargo test

# Форматування коду
format:
	cargo fmt

# Лінтинг коду з виведенням лише попереджень
lint:
	cargo clippy -- -D warnings

# Команда для перевірки перед комітом (форматування, лінтинг і тести)
precommit: format lint test

# Очищення проєкту
clean:
	cargo clean

# Оновлення залежностей
update:
	cargo update
