# 🔍 RustySearch（バックエンド）

**RustySearch** は、Rust + Actix + Diesel + FAISS によって構築された検索APIです。  
キーワード検索（全文一致）とセマンティック検索（意味ベース）の切り替えが可能な、軽量かつ拡張性のある検索バックエンドです。

🟢 実行ページ：[https://rustysearch.yonecoding.com](https://rustysearch.yonecoding.com)


## 🎥 RustySearch デモ（ローカル動作）

全文検索・ベクトル検索を切り替え可能な AI Powered Search のデモ動画です。  
Rust + Actix + PostgreSQL によるバックエンド処理を体感できます。

▶️ [YouTubeで見る](https://youtu.be/Tkg7qqwnWqk)


## 主な機能

- `/search?query=...&mode=keyword`  
  → PostgreSQL による部分一致検索（`ILIKE`）

- `/search?query=...&mode=semantic`  
  → FAISS によるベクトル検索 + ISBN による詳細情報取得

- `mode` パラメータで検索方法を切り替え可能

- Relevance Score（関連度スコア）による結果のソート表示

## 技術スタック

- **Rust**: 高速・安全なバックエンド構築言語
- **Actix-web**:  軽量かつ非同期なWebサーバ
- **Diesel** + **PostgreSQL**: ORM + RDBによるキーワード検索
- **FAISS**: 意味ベースのベクトル検索エンジン
- **r2d2**: DBコネクションプーリング
- **Docker**: コンテナ実行環境（対応済み）


## ディレクトリ構成

```txt
backend/
├── src/
│   ├── main.rs                  # サーバ起動・ルーティング構成
│   ├── routes/                 # ルーティングとHTTPエンドポイント
│   │   └── search.rs
│   ├── services/               # ビジネスロジック（db操作、relevance計算）
│   │   ├── db.rs
│   │   ├── relevance.rs
│   │   └── types.rs
│   ├── models.rs               # Diesel用モデル定義
│   └── schema.rs               # Dieselスキーマ自動生成
├── diesel.toml
├── Cargo.toml
├── Dockerfile
└── .env（手動で作成）
```

## API 使用方法

### エンドポイント

```http
GET /search?query=Rust&mode=keyword
GET /search?query=自然言語&mode=semantic
```

### レスポンス例
```
{
  "results": [
    {
      "isbn": "978-1234567890",
      "title": "Rustによる検索システム",
      "author": "Yoneyama",
      "relevance_score": 0.82
    },
    ...
  ],
  "elapsed_time": 0.034
}
```

## ビルド方法

RustySearch は Docker Compose を使用してワンコマンドで起動できます
```
docker compose up
```
- .env ファイルに PostgreSQL や FAISS に必要な環境変数を定義しておく必要があります。
- diesel setup や FAISS index の生成はあらかじめ済ませてください（将来的には自動化予定）。