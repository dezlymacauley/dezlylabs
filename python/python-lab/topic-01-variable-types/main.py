def main() -> None:
    # config_path: str | int = "/etc/app.conf"

    config_path: str = "/etc/app.conf"
    print(config_path)

    retry_count: int = 3
    print(retry_count)

    is_enabled: bool = True
    print(is_enabled)

    servers: list[str] = ["web01", "web02"]
    print(servers)

    # Dict stores data as key-value pairs
    # This means the variable type of the `key` is a `str`,
    # and the variable type of the `value` is an `int`.
    settings: dict[str, int] = {"port": 8080}
    print(settings)

    account_balance: float = 837.20
    print(account_balance)

if __name__ == "__main__":
    main()
