import sys
import json
from sklearn.linear_model import LinearRegression
import numpy as np


# ダミーデータ: 実際には事前にトレーニングされたモデルを読み込む
def load_model():
    # 単純なダミーモデル: 実際には保存されたモデルを読み込むべき
    model = LinearRegression()
    # ダミーデータでフィッティング（実際には事前にトレーニングされたモデルを使用する）
    X_train = np.array([[1, 1], [1, 2], [2, 2], [2, 3]])
    y_train = np.array([6, 8, 9, 11])
    model.fit(X_train, y_train)
    return model


def rank_results(results):
    model = load_model()

    # 特徴量エンジニアリング: 各結果の特徴量を抽出
    features = np.array(
        [[len(result["title"]), len(result["description"])] for result in results]
    )

    # ランキングスコアを予測
    scores = model.predict(features)

    # スコアを各結果に追加
    for result, score in zip(results, scores):
        result["relevance_score"] = score

    # スコアでソート
    ranked_results = sorted(results, key=lambda x: x["relevance_score"], reverse=True)
    return ranked_results


if __name__ == "__main__":
    input_data = json.loads(sys.argv[1])
    ranked_results = rank_results(input_data)
    print(json.dumps(ranked_results))
