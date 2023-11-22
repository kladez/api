set dotenv-load
set shell := ["bash", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

docker_compose_path := "docker"/"compose.dev.yaml"
migration_path := "src"/"infrastructure"/"database"/"migrations"

default:
	just --list  --unsorted

# cargo check
check *FLAGS:
	cargo clippy {{FLAGS}}
alias c := check

# rustfmt
format *FLAGS:
	cargo +nightly fmt {{FLAGS}}
alias f := format

# cargo run
run:
	cargo run
alias r := run

# bacon run
bacon:
	bacon run
alias b := bacon

# docker-build:
# 	docker build \
# 		--file docker/Dockerfile \
# 		--tag ${APP_NAME}:latest \
# 		.

# docker-run:
# 	docker run \
# 		--env-file .env \
# 		-it \
# 		--rm \
# 		--publish ${BIND_PORT}:80 \
# 		--env DATABASE_HOST="$(docker inspect -f '{{{{range .NetworkSettings.Networks}}{{{{.IPAddress}}{{{{end}}' ${APP_NAME}-database)" \
# 		${APP_NAME}:latest

# docker compose up --detach
docker-compose-up:
	docker compose \
		--file {{docker_compose_path}} \
		up \
		--build \
		--detach \
		--remove-orphans
alias dcu := docker-compose-up

# docker compose down
docker-compose-down:
	docker compose \
		--file {{docker_compose_path}} \
		down
alias dcd := docker-compose-down

# sqlx migrate add
add-migration:
	sqlx migrate add -r \
		--source {{migration_path}}
alias ma := add-migration

# sqlx migrate run
migrate:
	sqlx migrate run \
		--source {{migration_path}}
alias m := migrate

# sqlx migrate revert
migrate-revert:
	sqlx migrate revert \
		--source {{migration_path}}
alias mr := migrate-revert

# commitlint
commitlint COMMIT_EDITMSG:
	npx @commitlint/cli --edit "{{COMMIT_EDITMSG}}"
