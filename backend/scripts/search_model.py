# search_model.py
from sentence_transformers import SentenceTransformer
import numpy as np
from sklearn.metrics.pairwise import cosine_similarity
import json


def load_book_vectors(path="scripts/book_vectors.json"):
    with open(path, "r") as f:
        return json.load(f)


def search_similar_books(query, top_k=5):
    model = SentenceTransformer("all-MiniLM-L6-v2")
    query_vec = model.encode([query])

    book_data = load_book_vectors()
    book_vecs = np.array([entry["vector"] for entry in book_data])
    isbns = [entry["isbn"] for entry in book_data]

    similarities = cosine_similarity(query_vec, book_vecs)[0]

    results = [
        {"isbn": isbn, "relevance_score": float(score)}
        for isbn, score in zip(isbns, similarities)
    ]
    results.sort(key=lambda x: x["relevance_score"], reverse=True)

    return results[:top_k]
