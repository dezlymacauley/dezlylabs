from pathlib import Path

def main():
    # You can split the file path into sections 
    config_dir = Path("./configs")
    file_name = "settings.yaml"

    # Then use the `/` operator to join the sections into one file path
    config_path = config_dir / file_name
    print(config_path)
    # configs/settings.yaml

    # How to check if a path exists
    if config_path.exists() == True:
        print(f"The path: {config_path} was found")
    else:
        print(f"Error: The path: {config_path} was not found")
        return
    
    # How to check if this is a path to a directory or a file
    if config_path.is_file() == True:
        print(f"{config_path} is a path to a file")
    else:
        print(f"Error: {config_path} is not a path to a file")
        return


if __name__ == "__main__":
    main()
