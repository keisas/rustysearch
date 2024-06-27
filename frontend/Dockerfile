# ベースイメージ
FROM node:14

# 作業ディレクトリの設定
WORKDIR /app

# 依存関係をコピー
COPY package.json /app
COPY package-lock.json /app

# 依存関係をインストール
RUN npm install

# アプリケーションのソースコードをコピー
COPY . /app

# ビルド
RUN npm run build

# ビルドされたアプリケーションを提供するためにnginxをインストール
FROM nginx:alpine
COPY --from=0 /app/build /usr/share/nginx/html

# コンテナが起動したときにnginxを実行
CMD ["nginx", "-g", "daemon off;"]

# ポートを公開
EXPOSE 80