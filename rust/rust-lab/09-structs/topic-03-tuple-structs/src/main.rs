struct Numbers(i32, i32);

impl Numbers {
    fn largest_number(&self) {
        if self.0 > self.1 {
            println!("{} is larger than {}", self.0, self.1)
        } else if self.0 < self.1 {
            println!("{} is smaller than {}", self.0, self.1)
        } else {
            println!("{} is equal to {}", self.0, self.1);        
        }
    }
}

fn main() {
    let some_nums: Numbers = Numbers(16, 16);
    println!(
        "The values of the two fields are {} and {}",
        some_nums.0, some_nums.1
    );
    
    some_nums.largest_number();
}
