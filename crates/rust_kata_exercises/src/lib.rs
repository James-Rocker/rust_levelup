
pub fn flick_switch(list: &[&str]) -> Vec<bool> {
    // Kata - https://www.codewars.com/kata/64fbfe2618692c2018ebbddb
    let mut flick_val: bool = true;
    let mut return_list: Vec<bool> = Vec::new();
    for i in list {
        if i.eq(&"flick") {
            flick_val = !flick_val.clone();
        }
        return_list.push(flick_val.clone())
    }
    return return_list
}

pub fn find_short(s: &str) -> u32 {
    // Kata - https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9
    // This was my solution but I'm not a fan of it for several reasons
    let mut shortest_word: u32 = 99;

    for word in s.split_whitespace() {
        if word.chars().count() < shortest_word.try_into().unwrap() {
            println!("shortest word is: {}", word);
            shortest_word = word.chars().count() as u32;
        }
    }
    return shortest_word
}

pub fn find_short_better_solution(s: &str) -> u32 {
    // this is the solution I much preferred to mine
    // return s.split_whitespace()
    //     .min_by_key(|s| s.len())
    //     .unwrap()
    //     .len() as u32

    // breaking it down
    for each in s.split_whitespace() {
        // split_whitespace splits each word by whitespace and returns an iterator
        println!("each word {}", each);
    }

    // run min by key against an iterator taking a key based on a value. Then each value in the
    // iterator length is defined. Then take the complex enumerator type and get the result
    println!("Minimum by key {}", s.split_whitespace()
        .min_by_key(|s| s.len())
        .unwrap());

    // finally get the length as an unsigned integer
    return s.split_whitespace()
        .min_by_key(|s| s.len())
        .unwrap()
        .len() as u32
}
