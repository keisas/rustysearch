# ベースイメージとしてPostgreSQLの公式イメージを使用
FROM postgres:13

# 初期化スクリプトをコンテナ内の特定のディレクトリにコピー
COPY init.sql /docker-entrypoint-initdb.d/

# カスタム設定ファイルをコンテナ内のPostgreSQL設定ディレクトリにコピー
COPY custom.conf /etc/postgresql/postgresql.conf

# 環境変数を設定
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword
ENV POSTGRES_DB=mydatabase

# Postgresのデフォルトポートを公開
EXPOSE 5432