FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install sqlx-cli --no-default-features --features sqlite

RUN sqlx database create --database-url "sqlite:./database.db"
RUN sqlx migrate add -r create_users_table
RUN sqlx migrate run --database-url sqlite:./database.db

EXPOSE 8080

CMD ["cargo", "run"]
