# 🛠️ Development

dev-rust:
	docker run --rm -v $(PWD)/backend:/app -w /app --user 1000:1000 -it rust

gen-cert:
	docker run --rm -v $(PWD)/backend:/app -w /app/cert --user 1000:1000 rust \
		openssl req -x509 -newkey rsa:4096 -nodes -sha256 -keyout key.pem -out cert.pem -days 365 \
		-subj "/CN=localhost"

rust-serve:
	docker run -d --rm -p 3000:3000 -v $(PWD)/backend:/app -w /app --name rust-container rust cargo run

dev-angular:
	docker run --rm -v $(PWD)/frontend:/app -w /app --user 1000:1000 -it angular sh

angular-serve:
	docker run -d --rm -p 4200:4200 -v $(PWD)/frontend:/app -w /app --name angular-container dev-angular ng serve --host 0.0.0.0

db-serve:
	docker run -d --rm -p 5432:5432 \
		--env POSTGRES_USER=portfolio_manager \
		--env POSTGRES_PASSWORD=password \
		--env POSTGRES_DB=portfolio_manager \
		-v $(PWD)/data/db:/var/lib/postgresql/data:Z --name postgres-container postgres:17.5

serve-all: db-serve rust-serve angular-serve

# 🐳 Docker

build-dev-angular:
	docker build -t dev-angular docker/angular