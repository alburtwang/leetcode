use std::collections::{HashSet, HashMap};
use core::borrow::Borrow;

pub struct Solution{}

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {

        let morses = vec![".-","-...","-.-.","-..",".","..-.","--.","....","..",
                          ".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-",
                          "..-","...-",".--","-..-","-.--","--.."];

        let mut morsesWord = HashMap::new();
        let mut i = 0;

        for c in (0 .. 26).map(|x| (x + b'a') as char) {
            morsesWord.insert(c, morses.get(i).unwrap());
            i = i + 1;
        }
        let mut set = HashSet::new();

        for word in &words {
            let mut t = String::new();
            for c in word.chars() {
                let x = c.borrow();
                t.push_str(morsesWord.get(x).unwrap())
            }
            set.insert(t);
        }
        return set.len() as i32;
    }
}

fn main() {
    let words = vec![String::from("gin"), String::from("zen"), String::from("gig"), String::from("msg")];
    assert_eq!(2, Solution::unique_morse_representations(words));
}