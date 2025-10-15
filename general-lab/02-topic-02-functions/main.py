# Functions that don't return anything will return the type `None`
def say_hello() -> None:
    print("Hello world")


def get_server_status(hostname: str, port: int) -> None:
    print(f"hostname: {hostname}")
    print(f"port: {port}")


def main() -> None:
    say_hello()
    get_server_status("debian", 8080)


if __name__ == "__main__":
    main()
