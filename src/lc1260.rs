use crate::Solution;

/// # 1260. 二维网格迁移
/// 简单
/// 相关标签
/// premium lock icon
/// 相关企业
/// 提示
/// 给你一个 m 行 n 列的二维网格 grid 和一个整数 k。你需要将 grid 迁移 k 次。
/// 
/// 每次「迁移」操作将会引发下述活动：
/// 
/// 位于 grid[i][j]（j < n - 1）的元素将会移动到 grid[i][j + 1]。
/// 位于 grid[i][n - 1] 的元素将会移动到 grid[i + 1][0]。
/// 位于 grid[m - 1][n - 1] 的元素将会移动到 grid[0][0]。
/// 请你返回 k 次迁移操作后最终得到的 二维网格。
/// 
///  
/// 
/// ## 示例 1：
/// 
/// 
/// 
/// 输入：grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
/// 输出：[[9,1,2],[3,4,5],[6,7,8]]
/// ## 示例 2：
/// 
/// 
/// 
/// 输入：grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
/// 输出：[[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
/// ## 示例 3：
/// 
/// 输入：grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
/// 输出：[[1,2,3],[4,5,6],[7,8,9]]
///  
/// 
/// ## 提示：
/// 
/// m == grid.length
/// n == grid[i].length
/// 1 <= m <= 50
/// 1 <= n <= 50
/// -1000 <= grid[i][j] <= 1000
/// 0 <= k <= 100
/// 


impl Solution {
    
    // fn main() {
    //     let result = Solution::shift_grid(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]],1);
    //     println!("{:?}", result);
    // }


    ///  # 方法一：一维展开
    ///      设 m 和 n 分别为网格的行列数，我们将网格 grid 想象成由多个一维数组 {grid[i];0≤i<n} 按顺序拼接而成的一维数组，那么元素 grid[i][j] 在该一维数组的下标为 index=i×n+j。
    ///
    ///   每次迁移操作都相当于将该一维数组向右循环移动一次，那么 k 次迁移操作之后，元素 grid[i][j] 在该一维数组的下标变为 index(1)
    ///  =(index+k)mod(m×n)，在网格的位置变为 grid[⌊ index (1)/n​⌋][index(1)modn]。

    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let index1 = (i * n + j + (k as usize)) % (m * n);
                ret[index1 / n][index1 % n] = grid[i][j];
            }
        }
        return ret;
    }
}