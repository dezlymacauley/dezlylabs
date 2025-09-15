# This will import Python's built-in `json` package
import json

# This will import the external package `requests`
import requests

# This will import the variable type of a response
from requests import Response

def main() -> None:
    response: Response = requests.get("https://api.github.com", timeout=10)

    print(f"Status code: {response.status_code}")
    print(f"Content-Type: {response.headers.get("Content-Type")}")

    # print(".text attribute:")
    # print(response.text)
    # print()
    #
    # print(".content attribute:")
    # print(response.content)
    # print()
    #
    # print(".json() method:")
    # print(response.json())
    # print()

    data = response.json()
    print("Available Endpoints:")
    print(json.dumps(data, indent=2))

if __name__ == "__main__":
    main()
