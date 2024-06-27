import sys
import json


def calculate_relevance(articles):
    # 各記事に対してダミーのスコアを計算
    scores = [len(article["description"]) * 0.1 for article in articles]
    return scores


if __name__ == "__main__":
    articles = json.loads(sys.argv[1])
    scores = calculate_relevance(articles)
    print(json.dumps(scores))
