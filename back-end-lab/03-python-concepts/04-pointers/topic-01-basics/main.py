# WARNING: This is why I do not like Python
# Other languages do not behave like this.

# Python: Everything is an object reference.
# Small integers are cached and reused for efficiency.

# Other languages: Primitives are stored directly on the stack as values,
# each variable gets its own memory location.


def main():
    value_a = 11
    print(f"value_a is {value_a}")
    print(f"value_a is stored at the address {id(value_a):#014x}")
    # value_a is 11
    # value_a is stored at the address 0x0000017b3608

    value_b = value_a
    print(f"value_b is {value_b}")
    print(f"value_b is stored at the address {id(value_b):#014x}")
    # value_b is 22
    # value_b is stored at the address 0x0000017b3608

    value_b = 50
    print("value_b was updated to 50")

    print(f"value_a is {value_a}")
    print(f"value_a is stored at the address {id(value_a):#014x}")
    # value_a is 11
    # value_a is stored at the address 0x0000017b3608
    
    print(f"value_b is {value_b}")
    print(f"value_b is stored at the address {id(value_b):#014x}")
    # value_b is 50
    # value_b is stored at the address 0x0000017b3ae8

if __name__ == "__main__":
    main()
