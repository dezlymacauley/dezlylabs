# ABOUT: The f-string

# The syntax is:
# f"{name_of_variabl:optional_format_specifier}"


def main():
    value_a: int = 50

    print(f"value_a is {value_a}")
    # value_a is 50

    # E.g. I want to print the memory address of a variable
    # You can use the `id()` function
    # The default format of the `id()` function is decimal (base 10)
    print(f"value_a is stored at the address: {id(value_a)}")

    # To tell Python to display it as a hex value the same way 
    # as Rust, use `#014x`
    # `#` tells Python to include the prefix for hex, which is 0x
    # `0` tells Python to pad with zeros, to ensure that the number is not
    # shorter than than the total width (which will be specified next).
    # `14` means that that the total width 
    # should be 14 (0x, followed by 12 values)
    # `x` is the format specificier for hex

    print(f"value_a is stored at the address: {id(value_a):#014x}")
    # value_a is stored at the address: 0x0000017b3ae8

if __name__ == "__main__":
    main()
