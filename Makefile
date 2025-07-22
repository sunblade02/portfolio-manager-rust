# 🛠️ Development

dev-rust:
	docker run --rm -v $(PWD)/backend:/app -w /app --user 1000:1000 -it rust

dev-angular:
	docker run --rm -v $(PWD)/frontend:/app -w /app --user 1000:1000 -it angular sh

angular-serve:
	docker run -d --rm -p 4200:4200 -v $(PWD)/frontend:/app -w /app dev-angular ng serve --host 0.0.0.0

# 🐳 Docker

build-dev-angular:
	docker build -t dev-angular docker/angular