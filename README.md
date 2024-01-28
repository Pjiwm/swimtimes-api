# Swimtimes
![GitHub commit activity](https://img.shields.io/github/commit-activity/w/Pjiwm/swimtimes-api)
![GitHub top language](https://img.shields.io/github/languages/top/Pjiwm/swimtimes-api)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![React](https://img.shields.io/badge/react-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361DAFB)
![Vite](https://img.shields.io/badge/vite-%23646CFF.svg?style=for-the-badge&logo=vite&logoColor=white)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
## Available via

[Web UI](https://swimtimes-api.shuttleapp.rs/)

[GraphQL API](https://swimtimes-api.shuttleapp.rs/api/graphql)

## Installation

To install and run the Swimtimes application, follow these steps:

1. Install Docker, Node, and the Rust compiler.
2. Create a `secrets.toml` file for Shuttle deployment with the following content:

```toml
DATABASE_URL='mysql://root:1337@localhost:3306/swimdb'
RUST_LOG="info,sqlx=off,async_graphql=warn,api=info"
local_uri="postgres://postgres:postgres@localhost:5432/swimdb"
JWT_SECRET=
```

3. Install the `cargo-shuttle` tool by running:

```bash
cargo install cargo-shuttle
```

4. Setup a project on [Shuttle-rs](https://www.shuttle.rs/)

5. Setup [Supabase](https://supabase.com) or any other JWT/JWK provider

6. in `/swimtimes/app/app.config.ts` set the config
```ts
const config = {
  apiUrl: process.env.NODE_ENV === 'development' ? 'http://localhost:8000' : 'https://<url>.shuttleapp.rs',
  authClientUrl: "https://<id>.supabase.co",
  anonKey: "<anonkey>"
};

export default config;
```

7. Start the container using Docker Compose:

```bash
docker-compose up -d
```

8. Run the application with:

```bash
cargo shuttle run
```

## Application Overview

### Entities

#### Swimmer

| Field           | Type       | Description           |
| --------------- | ---------- | --------------------- |
| id              | i32        | Primary Key           |
| name            | String     | Swimmer's name        |
| date_of_birth   | Date       | Swimmer's date of birth|
| team            | i32 (FK)   | Foreign key to Team   |

#### Team

| Field           | Type       | Description           |
| --------------- | ---------- | --------------------- |
| id              | i32        | Primary Key           |
| name            | String     | Team's name           |
| founding_date   | Date       | Team's founding date  |
| address         | String     | Team's address        |
| zip_code        | String     | Team's zip code       |

#### SwimTime

| Field           | Type       | Description           |
| --------------- | ---------- | --------------------- |
| id              | i32        | Primary Key           |
| competition     | i32 (FK)   | Foreign key to Competition |
| distance        | i32        | Swim distance in meters|
| stroke          | String     | Swim stroke type      |
| time            | i32        | Swim time in seconds  |
| swimmer         | i32 (FK)   | Foreign key to Swimmer|

### Tech Stack

- Docker
- Node.js
- React
- Vite
- Rust
- Axum
- SQLx/Sea ORM
- GraphQL (async-graphql)
- Postgresql
- Supabase 

### Directory

```
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Secrets.toml
├── Shuttle.toml (use this instead of .env)
├── api (API library for Axum server)
├── app (React app)
├── docker-compose.yml
├── entities.sh (Migration command, creates entity files)
├── entity (Contains all models/entities for the project)
├── example.env
├── migration (Sea ORM migrations)
├── repository (manages commands to DB)
├── src (main crate, spins up web server from API using Shuttle-rs)
├── static (output directory for building react app. Will be served by API)
├── target
└── tests
```
