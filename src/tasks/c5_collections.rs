// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    let mut vec_unique = Vec::new();
    let mut unique_num = HashSet::new();
    for item in vec {
        if !unique_num.contains(item) {
            vec_unique.push(*item);
            unique_num.insert(item);
        }
    }

    vec_unique.sort();
    vec_unique.reverse();

    if vec_unique.len() >= 2 {
        Some(vec_unique[1])
    } else {
        None
    }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    let mut dp = Vec::new();
    let mut p: Vec<i32> = Vec::new();
    let n = init_sequence.len();

    for i in 0..n {
        dp.push(1);
        p.push(-1);
        for j in 0..i {
            if init_sequence[j] < init_sequence[i] && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                p[i] = j as i32;
            }
        }
    }

    let mut ans = dp[0];
    let mut pos = -1;
    for i in 0..n {
        if dp[i] > ans {
            ans = dp[i];
            pos = i as i32;
        }
    }

    let mut vec_res = Vec::new();
    while pos != -1 {
        vec_res.push(init_sequence[pos as usize]);
        pos = p[pos as usize];
    }
    vec_res.reverse();
    vec_res
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let mut vec_words: Vec<&str> = sentence.split(" ").collect();
    vec_words.reverse();
    vec_words.join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let sentence_new = sentence.trim().to_string();
    let words_in_sentence: Vec<String> = sentence_new
        .split_whitespace()
        .map(|word| {
            let mut word_chars = word.chars();
            let first = word_chars.next().unwrap().to_uppercase().to_string();
            let rest = word_chars.as_str().to_lowercase();
            first + rest.as_str()
        })
        .collect();
    words_in_sentence.join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut unique_set = HashSet::new();
    let mut res = true;
    for now_char in s.chars() {
        if unique_set.contains(&now_char) {
            res = false;
        }
        unique_set.insert(now_char);
    }
    res
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut hash_frequence = HashMap::<i32, i32>::new();
    for item in nums {
        let frequence = hash_frequence.entry(item).or_insert(0);
        *frequence += 1;
    }
    let mut sort_nums: Vec<i32> = hash_frequence.keys().cloned().collect();
    sort_nums.sort_by(|first, second| hash_frequence[first].cmp(&hash_frequence[second]));
    sort_nums.reverse();
    let mut res = Vec::new();
    let mut cnt = k;
    if cnt > sort_nums.len() {
        cnt = sort_nums.len();
    }

    for item in sort_nums.iter().take(cnt) {
        res.push(*item);
    }
    res
}
