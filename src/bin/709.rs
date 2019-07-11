use std::rc::Rc;
pub struct Solution{}

impl Solution {

    pub fn to_lower_case(str: Rc<String>) -> String {

//        let mut s = String::with_capacity(str.len());
//        let gap = i32('A') - 'a';
//        for c in str.chars() {
//            if c >= 'A' && c <= 'Z' {
//                s.push(c - gap);
//            }else {
//                s.push(c);
//            }
//        }


        return str.to_lowercase();

//        return s;
    }
}

fn main() {
    println!("hello, world!");
    let origin = Rc::new(String::from("ABC"));
    let param = origin.clone();
    let s = Solution::to_lower_case(param);
    println!("change from {:?} to {:?}", origin, s)
}