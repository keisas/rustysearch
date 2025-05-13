from sklearn.linear_model import LinearRegression
import numpy as np


# ダミーモデルの読み込み（将来は SentenceTransformer などに）
def load_model():
    model = LinearRegression()
    X_train = np.array([[1, 1], [1, 2], [2, 2], [2, 3]])
    y_train = np.array([6, 8, 9, 11])
    model.fit(X_train, y_train)
    return model


# 外部から呼ばれるスコア計算関数（Rust経由で）
def score_books(books):
    model = load_model()

    features = np.array(
        [[len(book["title"]), len(book.get("description", ""))] for book in books]
    )
    scores = model.predict(features)
    return scores.tolist()
