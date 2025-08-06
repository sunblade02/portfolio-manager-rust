# 💼 PortfolioManager (Rust)

PortfolioManager is a lightweight web application for managing and tracking financial assets, including cryptocurrencies.

## 🚀 Features

Coming soon... ⏳

## 🛠️ Technical Stack

- Frontend : Angular  
- Backend : Rust (AXUM)  
- Database : PostgreSQL  
- Deployment : Docker + Kubernetes

## 🗺️ Roadmap

Check the [roadmap here](./ROADMAP.md)

## 📘 User guide

Read the [user guide here](./USER_GUIDE.md)

## 🧱 Technical Architecture

Coming soon... ⏳

## 📦 Installation

### Requirements

- [Docker](https://www.docker.com/) must be installed on your system.
- Make sure `make` are available as well.

### Launch the project in development environment

#### 1. Generate SSL certificate for AXUM

```bash
make gen-cert
```

#### 2. Create the .env file in the backend directory

```bash
cp backend/.env.example backend/.env
```

#### 3. Start all development services

```bash
docker compose up -d
```

### Build the Angular development container

```bash
make build-dev-angular
```

### Run developement containers

#### Run the Rust development container

```bash
make dev-rust
```

#### Run the Angular development container

```bash
make dev-angular
```

More coming soon... ⏳

## ⚙️ Configuration

The .env file in the backend directory requires the following parameters :
- DATABASE_URL : connection string to the database
- JWT_SECRET : secret key used for generating of JWT tokens

## 📂 Project Structure

/
├── backend/                # Rust backend code
|   ├── cert/               # TLS certificate for Axum
|   ├── migrations/         # SQL migration scripts
|   ├── src/                # Source code
|   |   ├── bin/            # Executables (migration)
|   |   ├── handlers/       # HTTP handlers
|   |   ├── models/         # Database models
|   |   ├── services/       # Services
|   |   └── utils/          # Utility functions
|   └── tests/              # Test code
├── data/
|   └── db/                 # Database data files
├── docker/                 # Dockerfiles and related configurations
└── frontend/               # Angular frontend code
    ├── public/             # Static assets
    └── src/                # Source code
        ├── app/
        |   ├── core/       # Core services
        |   ├── features/   # Feature modules
        |   └── shared/     # Shared components
        └── environments/   # Environments configurations

## 🧪 Tests

### Run all tests

```bash
make tests
```

### Run backend tests

#### 1. Create the TEST database

If necessary, stop any running containers :

```bash
docker compose down
```

Then :

```bash
make create-db-test
```

#### 2. Run Rust tests

```bash
make test-backend
```

### Run frontend tests

```bash
make test-frontend
```

## 📄 License

This project is licensed under the MIT License.  
You are free to use, copy, modify, and distribute it with attribution.

See the [LICENSE](./LICENSE) file for more details.