
use crate::Solution;
/// # 1081. 不同字符的最小子序列
/// 中等
/// 相关标签
/// premium lock icon
/// 相关企业
/// 提示
/// 返回 s 字典序最小的子序列，该子序列包含 s 的所有不同字符，且只包含一次。
///
/// ## 示例 1：
/// 
/// 输入：s = "bcabc"
/// 输出："abc"
/// ## 示例 2：
/// 
/// 输入：s = "cbacdcbc"
/// 输出："acdb"
///  
/// 
/// 提示：
/// 
/// 1 <= s.length <= 1000
/// s 由小写英文字母组成
/// 
/// 
/// 
impl Solution {
    // fn main() {
    //     let result = Solution::smallest_subsequence(String::from("bcabc"));
    //     println!("{:?}", result);
    // }



    pub fn smallest_subsequence(s: String) -> String {
        let mut vis = [false; 26];
        let mut num = [0;26];
        let bytes = s.as_bytes();
        for (_, &ch) in bytes.iter().enumerate() {
            num[ (ch - (b'a' as u8)) as usize]+=1;
        }

        let mut stk = String::new();
        for (_, &ch) in bytes.iter().enumerate() {
            let index = (ch - (b'a' as u8)) as usize;
            if !vis[index]  {
                while !stk.is_empty() && stk.chars().last().unwrap() > (ch as char) {
                    if num[(stk.chars().last().unwrap() as usize )- (b'a' as usize)] > 0 {
                        vis[(stk.chars().last().unwrap() as usize)  - (b'a' as usize) ] = false;
                        stk.pop();
                    } else {
                        break;
                    }
                }
                vis[index] = true;
                stk.push(ch as char);
            }
            num[index] -= 1;

        }
        
        return stk;
    }

}