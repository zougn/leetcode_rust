
use leetcode::Solution;


fn main() {
    let v = Solution::sequential_digits( 10,514800930);
    // let v = Solution::sequential_digits( 89,234);
    for i in &v {
        println!("{}", i);
    }

}
