
use leetcode::Solution;


fn main() {
    let vec = vec![24,26,25,20,27,27,27,27,20,27,21,27,25,20,20,23,25,21,20,29,24,21,23,25,28,21,21,28,25,21];
    let result = Solution::subsequence_pair_count( vec);
    println!("{}", result);

}
