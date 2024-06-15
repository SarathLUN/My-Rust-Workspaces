# My Rust Workspace

- my chatGPT prompt for this project:

```text
i want you to act as professional Rust developer and trainer.
i will give you the project requirements and you will guide me step-by-step with the workable code snippets on how to implement to archive our goal.
- project goal: to implement CRUD api on arcticle with Actix-web framework, Diesel ORM, and connection pool to Postgres.
i have Postgres docker container running and it is accessible via: `psql postgres://arcticle_api:password@localhost/arcticle_db`.
we should use diesel tool to run database migration to simplify the schema management.
ask me more questions if you need more detail before providing me the solutions.
```

```text
yes, we will start from scratch and to use Actix-web version 4.7, Diesel version 2.
here is the schema of the arcticle table (id: as auto Uuid, title: as String, content: as Text, published_at: as current time stamp, is_published: as bool, is_deleted: as bool, deleted_at: as timestamp of marked deleted).
below should be the endpoints to be implemented:
- /post/create_post: to create post and return back the created uuid.
- /post/get_post/{uuid}: to return single post based on provided param uuid.
- /post/list_posts: to return all published posts that not yet deleted.
- /post/list_all_posts: to return all posts that not yet deleted.
- /post/list_deleted_posts: to return all deleted posts.
- /post/update_post/{uuid}: to update post based on provided (title, content, is_published, published_at)
- /post/delete_post/{uuid}: to delete post from the database.
- /post/remove_post/{uuid}: to mark post as deleted.
i already have Diesel CLI for postgres. let start the implementation.
```

- explored actix-web with diesel and sqlite.
- explored actix-web with diesel and postgresql.
- start postgresql container:

```shell
docker run -d --name article_db -p 5432:5432 -e POSTGRES_USER=article_api -e POSTGRES_PASSWORD=password -e POSTGRES_DB=article_db -e PGDATA=/var/lib/postgresql/data/pgdata -v ./database/mounts:/var/lib/postgresql/data postgres:16.3-alpine
```

- create new project

```shell
cargo new article_api --bin
cd article_api
echo "DATABASE_URL=postgres://arcticle_api:password@localhost/arcticle_db" >> .env
diesel setup
diesel migration generate create_articles
```

- start the article api service:

```shell


cargo run --bin article_api
```

- test all endpoints:

```shell # create a new post:
curl -X POST http://localhost:8080/post/create_post \
    -H "Content-Type: application/json" \
    -d '{
        "title": "My First Article 2",
        "content": "This is my second article.",
        "is_published": true
    }'

```

```shell # Get a Single Post by UUID
curl -X GET http://localhost:8080/post/get_post/c15d271d-a73c-4309-bc2b-30ba53d49fb0

```

```shell # List All Published Posts that are Not Deleted
curl -X GET http://localhost:8080/post/list_posts

```

```shell # List All Posts that are Not Deleted
curl -X GET http://localhost:8080/post/list_all_posts

```

```shell # List All Deleted Posts
curl -X GET http://localhost:8080/post/list_deleted_posts

```

```shell # Update a Post by UUID
curl -X PUT http://localhost:8080/post/update_post/c15d271d-a73c-4309-bc2b-30ba53d49fb0 \
    -H "Content-Type: application/json" \
    -d '{
        "title": "Updated Title",
        "content": "Updated content of the article.",
        "is_published": true,
        "published_at": "2023-01-01T00:00:00"
    }'

```

```shell # Mark a Post as Deleted by UUID
curl -X DELETE http://localhost:8080/post/remove_post/c15d271d-a73c-4309-bc2b-30ba53d49fb0

```

```shell # Delete a Post by UUID
curl -X DELETE http://localhost:8080/post/delete_post/c15d271d-a73c-4309-bc2b-30ba53d49fb0

```
