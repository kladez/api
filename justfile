# Run local infrastructure
docker:
	docker compose -f docker/docker-compose.dev.yaml -p kladez up -d
	while ! docker logs kladez-postgres-1 2>&1 | grep -q "database system is ready to accept connections"; do sleep 1; done
	just sqlx-run

# Stop and remove local infrastructure
docker-down:
	docker compose -f docker/docker-compose.dev.yaml -p kladez down

# Restart local infrastructure
docker-restart:
	just docker-down
	just docker

# Add migration
sqlx-add NAME:
	sqlx migrate add --source src/infrastructure/database/migrations -r {{NAME}}

# Run migrations
sqlx-run:
	sqlx migrate run --source src/infrastructure/database/migrations

# Revert migrations
sqlx-revert:
	sqlx migrate revert --source src/infrastructure/database/migrations

# Generate query metadata to support offline compile-time verification
sqlx-prepare:
	cargo sqlx prepare

# Run tests
test:
	while ! just sqlx-revert | grep -q "No migrations available to revert"; do :; done
	just sqlx-run
	cargo nextest run
