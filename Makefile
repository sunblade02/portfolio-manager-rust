# 🛠️ Development

dev-rust:
	docker run --rm --network portfolio-manager-rust_default -v $(PWD)/backend:/app -w /app --user 1000:1000 -it rust

gen-cert:
	docker run --rm -v $(PWD)/backend:/app -w /app/cert --user 1000:1000 rust \
		openssl req -x509 -newkey rsa:4096 -nodes -sha256 -keyout key.pem -out cert.pem -days 365 \
		-subj "/CN=localhost"

dev-angular:
	docker run --rm -v $(PWD)/frontend:/app -w /app --user 1000:1000 -it angular sh

migrate-db:
	docker run --rm --network portfolio-manager-rust_default -v $(PWD)/backend:/app -w /app rust cargo run --bin migrate --no-default-features --features migrate

# 🧪 Test

create-db-test:
	docker run --rm -v $(PWD)/data/db:/var/lib/postgresql/data:Z -u postgres postgres:17.5 bash -c "\
		pg_ctl -D /var/lib/postgresql/data start && \
		until pg_isready -h localhost; do sleep 1; done; \
		psql -U portfolio_manager -c 'CREATE DATABASE portfolio_manager_test;'"

test-backend:
	docker compose up -d
	docker run --rm --network portfolio-manager-rust_default -v $(PWD)/backend:/app -w /app rust bash -c "\
		cargo run --bin migrate --no-default-features --features migrate test && \
		cargo test -- --test-threads=1"

test-frontend:
	docker run --rm -v $(PWD)/frontend:/app -w /app angular ng test --watch=false

tests: 
	docker compose down
	-@make create-db-test
	@make test-backend 
	@make test-frontend

# 🐳 Docker

build-dev-angular:
	docker build -t dev-angular docker/angular