
pub struct Solution{}

impl Solution {

    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {


        a.into_iter()
            .map(|mut v| {
                v.reverse();
                v.into_iter()
                    .map(|element| element ^ 1)
                    .collect::<Vec<i32>>()
            })
            .collect()
    }

}

fn main() {

    let v1 = vec![1, 1, 0];
    let v2 = vec![1, 0, 1];
    let v3 = vec![0, 0, 0];

    let v = vec![v1, v2, v3];


    println!("{:?}", v);
    let v1 = Solution::flip_and_invert_image(v);
    println!("{:?}", v1);
}