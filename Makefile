build:
	cargo build

run:
	watchexec -e rs -r cargo run

fix:
	cargo fix --bin "actixAPI"

makemigrations:
	sqlx migrate add -r init

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

install:
	cargo add reqwest
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres uuid"
	cargo add argon2
	cargo add rand_core --features "std"
	cargo install sqlx-cli
	cargo install --locked watchexec-cli