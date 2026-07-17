use std::collections::HashMap;

use crate::Solution;


///2981. 找出出现至少三次的最长特殊子字符串 I
///中等
///相关标签
///premium lock icon
///相关企业
///提示
///给你一个仅由小写英文字母组成的字符串 s 。
///
///如果一个字符串仅由单一字符组成，那么它被称为 特殊 字符串。例如，字符串 "abc" 不是特殊字符串，而字符串 "ddd"、"zz" 和 "f" 是特殊字符串。
///
///返回在 s 中出现 至少三次 的 最长特殊子字符串 的长度，如果不存在出现至少三次的特殊子字符串，则返回 -1 。
///
///子字符串 是字符串中的一个连续 非空 字符序列。
///
/// 
///
///示例 1：
///
///输入：s = "aaaa"
///输出：2
///解释：出现三次的最长特殊子字符串是 "aa" ：子字符串 "aaaa"、"aaaa" 和 "aaaa"。
///可以证明最大长度是 2 。
///示例 2：
///
///输入：s = "abcdef"
///输出：-1
///解释：不存在出现至少三次的特殊子字符串。因此返回 -1 。
///示例 3：
///
///输入：s = "abcaba"
///输出：1
///解释：出现三次的最长特殊子字符串是 "a" ：子字符串 "abcaba"、"abcaba" 和 "abcaba"。
///可以证明最大长度是 1 。
/// 
///
///提示：
///
///3 <= s.length <= 50
///s 仅由小写英文字母组成。

impl Solution {

    // fn main() {
    //     let result = Solution::maximum_length(String::from("aaaa") );
    //     println!("{}", result);
    // }



    // pub fn maximum_length(s: String) -> i32 {

    //     let mut map = HashMap::new();
    //     for i in 0..s.len(){
    //         for j in i..s.len(){
    //             if &s[i..i+1] != &s[j..j+1]{
    //                 break;
    //             }
    //             map.entry(&s[i..j+1])
    //                 .and_modify(|count| *count += 1)
    //                 .or_insert(1);
    //         }
    //     }

    //     let mut  max = -1;
    //     for (k,v) in &map{
    //         if *v>=3 && k.len() as i32 >max {
    //             max = k.len() as i32;
    //         }
    //     }
        
    //     return max;
    // }


    /// 方法一：一次遍历
    /// 思路
    /// 因字符串仅含有小写字母，所以可以对每种字母单独处理。对于每一种字母，统计出每部分连续子串的长度，并储存在数组 chs 中。因题目要求出现至少三次，因此只要维护前三大的长度即可。每次往 chs 中添加元素时，可采用冒泡的方法使其有序。如果长度超过 3，则将末尾元素 pop 掉。
    /// 更新答案时，主要有三种：
    /// 最长的 chs[0] 可贡献出 3 个长为 chs[0]−2 的子串，并且需要满足 chs[0]>2。
    /// 当 chs[0] 与 chs[1] 相等时，可贡献出 4 个长为 chs[0]−1 的子串。不等时可由 chs[0] 贡献出 2 个长为 chs[1] 的子串，加上 chs[1] 本身一共 3 个，并且需要满足 chs[0]>1。
    /// 可由 chs[0] 与 chs[1] 加上 chs[2] 本身贡献 3 个长为 chs[2] 的子串。
    ///
    ///没有更新答案时，则输出 −1。
    pub fn maximum_length(s: String) -> i32 {
        let mut ans = -1;
        let len = s.len();
        let mut chs: Vec<Vec<i32>> = vec![vec![]; 26];
        let mut cnt = 0;
        let s_bytes = s.as_bytes();

        for i in 0..len {
            cnt += 1;
            if i + 1 == len || s_bytes[i] != s_bytes[i + 1] {
                let ch = (s_bytes[i] - b'a') as usize;
                chs[ch].push(cnt);
                cnt = 0;
                for j in (1..chs[ch].len()).rev() {
                    if chs[ch][j] > chs[ch][j - 1] {
                        chs[ch].swap(j, j - 1);
                    } else {
                        break;
                    }
                }
                if chs[ch].len() > 3 {
                    chs[ch].pop();
                }
            }
        }
        println!("{:?}",chs);

        for i in 0..26 {
            if chs[i].len() > 0 && chs[i][0] > 2 {
                ans = ans.max(chs[i][0] - 2);
            }
            if chs[i].len() > 1 && chs[i][0] > 1 {
                ans = ans.max((chs[i][0] - 1).min(chs[i][1]));
            }
            if chs[i].len() > 2 {
                ans = ans.max(chs[i][2]);
            }
        }

        ans
    }

}