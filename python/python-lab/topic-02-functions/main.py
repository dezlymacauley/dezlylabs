# Functions that don't return anything will return the type `None`
def say_hello() -> None:
    print("Hello world")


say_hello()

# ______________________________________________________________________________


def get_server_status(hostname: str, port: int) -> None:
    print(f"hostname: {hostname}")
    print(f"port: {port}")


get_server_status("debian", 8080)

# ______________________________________________________________________________
