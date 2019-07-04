use std::string::String;
use std::collections::HashMap;

fn main() {

    let j = String::from("aA");
    let s = String::from("aAAbbbb");

    println!("{}", num_jewels_in_stones(j, s));
}

// solution 1
//fn num_jewels_in_stones(j: String, s:String) -> i32 {
//    let mut count = 0;
//    for c in j.chars() {
//        for k in s.chars() {
//            if c == k {
//                count = count + 1;
//            }
//        }
//    }
//    return count;
//}

// solution 2
fn num_jewels_in_stones(j: String, s:String) -> i32 {
    let mut count = 0;
    let mut m = HashMap::new();
    for c in j.chars() {
       m.insert(c, 0);
    }

    for c in s.chars() {
        if m.contains_key(&c) {
            count = count + 1;
        }
    }

    return count;
}