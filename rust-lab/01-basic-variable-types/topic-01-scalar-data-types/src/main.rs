/*

ABOUT: Decimal

Decimal (aka base 10) is the numeric system that humans use.

Base 10 uses 10 unique digits: 0, 1, 2, 3, 4, 5, 6, 7, 8, and 9


ABOUT: Binary 

Binary (aka base 2) is the numeric system that computers use.

Base 2 uses only 2 unique digits: 0 and 1

This makes sense for computers because if you think about 
an electrical switch, it has two states:

Off (represented by 0)
On (represented by 1)


ABOUT: Bits and Bytes

| Position          | 0  | 1  | 2  | 3  | 4  | 5  | 6  | 7   |
|-------------------|----|----|----|----|----|----|----|-----|
| **Power of 2**    | 2⁰ | 2¹ | 2² | 2³ | 2⁴ | 2⁵ | 2⁶ | 2⁷  |
| **Decimal Value** | 1  | 2  | 4  | 8  | 16 | 32 | 64 | 128 |

1 byte = 8 bits

*/

//_________________________________________________________________________

fn main() {
    
    //_________________________________________________________________________
    // SECTION: Integers

    // i8, i16, i32, i64

    // The default is i32

    //_________________________________________________________________________
    // SECTION: Unsigned Integers

    // u8, u16, u32, u64

    // The default is u32

    //_________________________________________________________________________
    // SECTION: Floating Points

    // f32, f64

    // The default is f64

    //_________________________________________________________________________
    // SECTION: Bool (true or false)

    let is_premium_member: bool = true;

    let has_profile_pic: bool = false;

    println!("is_premium_member: {is_premium_member}");
    println!("has_profile_pic: {has_profile_pic}");
    // is_premium_member: true
    // has_profile_pic: false

    //_________________________________________________________________________
    // SECTION: How to check what the smallest and largest number an integer
    // type can store

    // If you want to know what is smallest number,
    // and the largest number that a specific type can store,
    // then do this:

    println!("MIN of i8: {}", std::i8::MIN);
    println!("MAX of i8: {}", std::i8::MAX);
    // MIN of i8: -128
    // MAX of i8: 127
    //_________________________________________________________________________
    // SECTION: Char
   
    // You must use a single qoute.
    let my_char: char = 'a';

    //_________________________________________________________________________
    // SECTION: How to check large (size in bytes) a variable type is

    println!("Size of i8: {} bytes", std::mem::size_of::<i8>());
    println!("Size of i16: {} bytes", std::mem::size_of::<i16>());
    println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("Size of i64: {} bytes", std::mem::size_of::<i64>());
    // Size of i8: 1 bytes
    // Size of i16: 2 bytes
    // Size of i32: 4 bytes
    // Size of f64: 8 bytes

    //_________________________________________________________________________
}
