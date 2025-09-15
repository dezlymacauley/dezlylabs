# config_path: str | int = "/etc/app.conf"

config_path: str = "/etc/app.conf"

retry_count: int = 3

is_enabled: bool = True

servers: list[str] = ["web01", "web02"]

# Dict stores data as key-value pairs
# This means the variable type of the `key` is a `str`,
# and the variable type of the `value` is an `int`.
settings: dict[str, int] = {"port": 8080}

account_balance: float = 837.20

print(account_balance)
