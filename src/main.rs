
use std::vec;

use leetcode::Solution;


fn main() {
    let result = Solution::shift_grid(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]],1);
    println!("{:?}", result);
}
