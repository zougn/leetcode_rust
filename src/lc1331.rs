use crate::Solution;
use std::collections::HashMap;


impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut clone = arr.clone();

        clone.sort_unstable();
        let mut dd = 0;
        let mut map = HashMap::new();
        for i in &clone {
            if !map.contains_key(i){
                dd+=1;
                map.insert(*i,dd);
            }
        }
        let mut new1 = Vec::new();
        for i in &arr {
            if let Some(val) = map.get(i){
                new1.push(*val);
            }
        }

        return new1;
    }

}

// fn main() {
//     let v = Solution::array_rank_transform( vec![100, 32, 57]);
//     for i in &v {
//         println!("{}", i);
//     }
// }
