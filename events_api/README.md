- start postgres container with `events_db`
```shell
docker run -d --name events_db -p 5432:5432 -e POSTGRES_USER=events_api -e POSTGRES_PASSWORD=events_api -e POSTGRES_DB=events_db -e PGDATA=/var/lib/postgresql/data/pgdata -v ./database/mounts:/var/lib/postgresql/data postgres:16.3-alpine
```

- create a new project:
```shell
cargo new events_api
cd events_api
cargo add actix-web
cargo add serde -F derive
cargo add diesel -F postgres -F r2d2 -F chrono -F uuid
cargo add dotenv
cargo add env_logger
cargo add r2d2
cargo add uuid -F v4
cargo add chrono -F serde
cargo add diesel_migrations -F postgres
```

- generate migration
```shell
diesel migration generate create_events
```

- run migration
```shell
diesel migration run
```
