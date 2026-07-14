
use crate::Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        match Self::find_first(low) {
            (Some(mut value), mut base) => {
                if value < low {
                    if value < 123456789 {
                        // 【关键修复】继续获取下一个值和新的base
                        if let (Some(next_val), _base) = Self::next_number_opt(value, base) {
                            value = next_val;
                             base=_base;
                        } else {
                            return result;
                        }
                    } else {
                        return result;
                    }
                }
                while value <= high {
                    result.push(value);
                    if let (Some(next_val), _base)  = Self::next_number_opt(value, base) {
                        value = next_val;
                        base=_base;
                    } else {
                        break;
                    }
                }
                result
            }
            _ => result,
        }
    }

    pub fn find_first(low: i32) -> (Option<i32>, i32) {
       // 计算 low 的位数
        let len = Self::number_len(low);
        // 优化3: 用 pow 替代循环计算基准值 b
        let base = 10_i32.pow((len - 1) as u32);
        // 获取最高位数字
        let start_digit = low / base;
        let candidate = Self::gen_number(start_digit, len);
        
        match candidate {
            Some(val) => {
                // 如果生成的数字太大，需要增加位数
                if val > 9 * base {
                    (candidate, base * 10)
                } else {
                    (candidate, base)
                }
            }
            None => (None, base),
        }
    }

    pub fn number_len(mut n: i32) -> u8 {
       let mut len: u8 = 0;
        while n > 0 {
            n /= 10;
            len += 1;
        }
        len
    }

    pub fn gen_number(start_digit: i32, length: u8) -> Option<i32> {
        if start_digit > 9 || length == 0 {
            return None;
        }
        
        let mut current = start_digit;
        
        for next_digit in (start_digit + 1)..=(start_digit + length as i32 - 1) {
            if next_digit > 9 {
                return if length < 9 {
                    Self::gen_number(1, length + 1)
                } else {
                    None
                };
            }
            current = current * 10 + next_digit;
        }
        Some(current)
    }

    pub fn next_number_opt( val: i32, mut base: i32) -> (Option<i32>, i32) {
        if base == 0 {
            return (None,base);
        }

       let val = val % base;
        let last_digit = val % 10;

        if last_digit == 9 {
            base *= 10;
            (Self::gen_number(1, Self::number_len(base)),base)
        } else {
            (Some(val * 10 + last_digit + 1), base)
        }
    }
}
