# Rusty-Book-Manager

参考：https://github.com/rust-web-app-book/rusty-book-manager

## コマンド

### 基本

```sh
cargo make build
```

### DB マイグレーション

#### マイグレーションファイル生成

```sh
sqlx migrate add -r start --source adapter/migrations
```

#### マイグレーション

```sh
cargo make migrate
```

#### リバート

```sh
cargo make sqlx migrate revert
```
