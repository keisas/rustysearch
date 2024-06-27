# ビルド環境の設定
FROM rust:1.79 as builder
WORKDIR /app
COPY . .

# Debianの依存関係をインストール
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# 環境変数を設定
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# 依存関係をインストールしてアプリケーションをビルド
RUN cargo build --release

# ポートを公開
EXPOSE 8080

# アプリケーションを実行
CMD ["./target/release/backend"]