// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.
#[allow(dead_code)]
pub fn string_ownership() {
    pub fn longest_owned(s1: String, s2: String) -> String {
        if s1.len() >= s2.len() { s1 } else { s2 }
    }
    let string1 = String::from("Hello");
    let string2 = String::from("world!");

    let res = longest_owned(string1, string2);
    println!("{}", res);
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.
#[allow(dead_code)]
pub fn simple_borrowing() {
    pub fn print_length(s: &str) {
        println!("{}", s.len());
    }
    let str1 = String::from("Hello world!");
    print_length(&str1);
    println!("{}", str1);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.
#[allow(dead_code)]
pub fn hard_borrowing() {
    pub fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
        string.push_str(suffix);
        string.len()
    }
    let mut str1 = String::from("Hello");
    let suffix = " word";
    let size1 = append_and_return_length(&mut str1, suffix);
    println!("{}", size1);
    let size2 = append_and_return_length(&mut str1, suffix);
    println!("{}", size2);
    println!("{}", str1);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.
pub fn last_word(slice: &str) -> &str {
    let words_in_slice: Vec<&str> = slice.split(" ").collect();
    let mut last_word = "";
    for item in words_in_slice.iter() {
        if !item.is_empty() {
            last_word = item;
        }
    }
    last_word
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.
pub fn longest_word(sentence: &str) -> &str {
    let words_in_sentence: Vec<&str> = sentence.split(" ").collect();
    let mut max_word = "";
    let mut max_size = 0;
    for item in words_in_sentence.iter() {
        if max_size <= item.len() {
            max_size = item.len();
            max_word = item;
        }
    }
    max_word
}
