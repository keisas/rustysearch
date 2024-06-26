# search_model.py
import pandas as pd
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import linear_kernel
import sys
import json

# サンプルデータ
data = [
    {"id": 1, "text": "Machine learning is fascinating."},
    {"id": 2, "text": "Artificial intelligence and machine learning."},
    {"id": 3, "text": "Deep learning applications."},
]

# データフレームに変換
df = pd.DataFrame(data)

# TF-IDFベクトライザーの設定
tfidf = TfidfVectorizer(stop_words="english")
tfidf_matrix = tfidf.fit_transform(df["text"])


def search(query):
    query_tfidf = tfidf.transform([query])
    cosine_similarities = linear_kernel(query_tfidf, tfidf_matrix).flatten()
    related_docs_indices = cosine_similarities.argsort()[:-5:-1]
    results = df.iloc[related_docs_indices]
    res = list(results["text"])
    return res


if __name__ == "__main__":
    query = sys.argv[1]
    results = search(query)
    print(json.dumps(results))
