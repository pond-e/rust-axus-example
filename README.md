## 環境構築
```bash
$ cargo install sqlx-cli --no-default-features --features sqlite
$ cargo add sqlx --features "sqlite runtime-tokio-native-tls chrono"
$ cargo add tokio --features=full
$ cargo add dotenv
```

## データベースの作成
```bash
$ sqlx database create --database-url "sqlite:./database.db"
```

## マイグレーションの作成・実行
```bash
$ sqlx migrate add -r create_users_table
```
```bash
$ sqlx migrate run --database-url sqlite:./database.db
```
確認
```bash
$ sqlite3 database.db 
SQLite version 3.42.0 2023-05-16 12:36:15
Enter ".help" for usage hints.
sqlite> .tables
_sqlx_migrations  users           
sqlite> pragma table_info(users);
0|id|INTEGER|0||1
1|name|TEXT|1||0
2|email|TEXT|1||0
3|address|TEXT|0||0
4|created_at|DATETIME|1|CURRENT_TIMESTAMP|0
```

## docker
```bash
$ docker run -p 8080:8080 -it --name rust-web-3 rust-web-2
```
