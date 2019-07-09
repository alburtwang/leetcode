
struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut ans = String::from("");
        let mut index = 0;
        for c in s.chars() {
           if index == 0 {
               index = index + 1;
               continue
           }else {
               if c == '(' {
                   index = index + 1;
               }else {
                   index = index - 1;
               }
               if index != 0 {
                   ans.push(c);
               }
           }
        }
        return ans
    }
}

fn main() {
    println!("hello, world!")
}