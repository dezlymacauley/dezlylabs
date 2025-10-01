/*
    ABOUT: The purpose of this program

    To read through a text file and count the occurrences of certain
    commmon words and then print out the total for each word.
*/

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    time::{Duration, Instant},
};

fn main() {
    let common_words: Vec<&str> = vec![
        "the", "be", "to", "of", "and", "a", "in", "that", "have", "I",
    ];

    let total_start: Instant = Instant::now();

    // .iter() returns &&str (a reference to a reference)
    // So .cloned() turns each &&str into an owned reference,
    // so think of it as removing one layer of reference.
    // So .iter().cloned() returns &str
    for common_word in common_words.iter().cloned() {
        // This line captures the moment that the program reached this line.
        let start = Instant::now();

        // Open the file
        // This creates a file object that represents the open file
        let file: File = File::open("/home/dezlymacauley/pg123.txt")
            .expect("Could not open file");

        // This reads the file line by line.
        let reader: BufReader<File> = io::BufReader::new(file);

        let mut count: i32 = 0;

        for line in reader.lines() {
            let line: String = line.expect("Could not read line");
            let words = line.split_whitespace();

            for word in words {
                if word.to_lowercase() == common_word {
                    count += 1;
                }
            }
        }

        let duration: Duration = start.elapsed();
        println!(
            "word {common_word} has a count of {count} and duration of {}",
            duration.as_millis()
        );
    }

    let duration: Duration = total_start.elapsed();
    println!("Total duration: {}", duration.as_millis());

    // word the has a count of 40379 and duration of 544
    // word be has a count of 2275 and duration of 543
    // word to has a count of 13590 and duration of 545
    // word of has a count of 19869 and duration of 544
    // word and has a count of 14468 and duration of 544
    // word a has a count of 14278 and duration of 542
    // word in has a count of 11024 and duration of 544
    // word that has a count of 7249 and duration of 545
    // word have has a count of 2721 and duration of 545
    // word I has a count of 0 and duration of 542
    // Total duration: 5443

}
