import requests
import json


def fetch_book_data(isbn):
    response = requests.get(
        f"https://openlibrary.org/api/books?bibkeys=ISBN:{isbn}&format=json&jscmd=data"
    )
    return response.json()


def main():
    isbns = [
        "9780140328721",
        "9780451526534",
        # 追加のISBN
    ]

    data = [fetch_book_data(isbn) for isbn in isbns]

    with open("books.json", "w") as f:
        json.dump(data, f)


if __name__ == "__main__":
    main()
