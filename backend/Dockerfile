# ビルド環境の設定
FROM rust:1.79 as builder
WORKDIR /app
COPY . .

# Debianの依存関係をインストール
# Debianパッケージの更新・必要ツール追加（Python含む）
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    python3 \
    python3-pip \
    python3-venv \
    && apt-get clean

# Pythonパッケージのインストール（transformers + sklearn）
RUN pip3 install --no-cache-dir --break-system-packages \
    sentence-transformers \
    scikit-learn \
    numpy

# 環境変数を設定
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# 依存関係をインストールしてアプリケーションをビルド
RUN cargo build --release

# ポートを公開
EXPOSE 8080

# アプリケーションを実行
CMD ["./target/release/backend"]