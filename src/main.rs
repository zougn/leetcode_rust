use leetcode::Solution;


fn main() {
    let result = Solution::max_active_sections_after_trade(String::from("01"),vec![vec![0,1]]);
    println!("{:?}", result);
}
