import sys
import json
from search_model import score_books


def main():
    books = json.loads(sys.argv[1])
    scores = score_books(books)  # ベクトル化して類似度 or ダミー
    print(json.dumps(scores))


if __name__ == "__main__":
    main()
