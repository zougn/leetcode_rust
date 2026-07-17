/// # 3658. 奇数和与偶数和的最大公约数
/// 简单
/// 相关标签
/// premium lock icon
/// 相关企业
/// 提示
/// 给你一个整数 n。请你计算以下两个值的 最大公约数（GCD）：
/// 
/// sumOdd：最小的 n 个正奇数的总和。
/// 
/// sumEven：最小的 n 个正偶数的总和。
/// 
/// 返回 sumOdd 和 sumEven 的 GCD。
/// 
///  
/// 
/// ## 示例 1：
/// 
/// 输入： n = 4
/// 
/// 输出： 4
/// 
/// 解释：
/// 
/// 前 4 个奇数的总和 sumOdd = 1 + 3 + 5 + 7 = 16
/// 前 4 个偶数的总和 sumEven = 2 + 4 + 6 + 8 = 20
/// 因此，GCD(sumOdd, sumEven) = GCD(16, 20) = 4。
/// 
/// ## 示例 2：
/// 
/// 输入： n = 5
/// 
/// 输出： 5
/// 
/// 解释：
/// 
/// 前 5 个奇数的总和 sumOdd = 1 + 3 + 5 + 7 + 9 = 25
/// 前 5 个偶数的总和 sumEven = 2 + 4 + 6 + 8 + 10 = 30
/// 因此，GCD(sumOdd, sumEven) = GCD(25, 30) = 5。
/// 
///  
/// 
/// ## 提示：
/// 
/// 1 <= n <= 1000

use crate::Solution;
impl Solution {

    // fn main() {
    //     let result = Solution::gcd_of_odd_even_sums( 4);
    //     println!("{}", result);
    // }


    ///由于 gcd(n**2,n×(n−1))=n×gcd(n,n+1)，又 n 与 n+1 互质，因此 gcd(n,n+1)=1，故 gcd(n**2,n×(n+1))=n
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
       n
    }
    // pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    //     let sumOdd = Self::sumOdd(1,n);
    //     let sumEven = Self::sumOdd(2,n);
    //     println!("{}",sumOdd);
    //     println!("{}",sumEven);
        
    //     return  Self::gcd(sumOdd,sumEven);
    // }

    // fn sumOdd(mut a: i32, n: i32)-> i32{
    //     let mut sum = a;
    //     for _ in 1..n{
    //         a+=2;
    //         sum +=a;
    //     }
    //     sum
    // }

    // fn gcd(mut a: i32, mut b: i32) -> i32 {
    //     while b != 0 {
    //         let temp = a;
    //         a = b;
    //         b = temp % b;
    //     }
    //     a
    // }
}