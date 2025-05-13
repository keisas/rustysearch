import sys
import json
from search_model import search_similar_books


def main():
    if len(sys.argv) < 2:
        print("[]")
        return

    query = sys.argv[1]
    results = search_similar_books(query)
    print(json.dumps(results))


if __name__ == "__main__":
    main()
#
