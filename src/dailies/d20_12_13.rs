

struct Solution;

//problem 312
use std::collections::HashMap;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        if nums.is_empty(){
            return 0;
        }
        let mut dp = HashMap::new();
        Solution::max_coins_recurse(&nums, &mut dp)
    }

    fn max_coins_recurse(nums: &[i32], dp: &mut HashMap<Vec<i32>, i32>) -> i32 {
        match nums.len() {
            1 => nums[0],
            2 => nums[0] * nums[1] + nums[0].max(nums[1]),
            _ => {
                let mut max = 0;
                for i in 0..nums.len() {
                    let slice = [&nums[0..i], &nums[i + 1..]].concat() as Vec<i32>;

                    let mut score = 1;
                    if i != 0 {
                        score *= nums[i - 1];
                    }
                    score *= nums[i];
                    if i != (nums.len() - 1) {
                        score *= nums[i + 1];
                    }

                    if dp.contains_key(&slice) {
                        score += dp[&slice];
                    } else {
                        let sub_score = Solution::max_coins_recurse(&slice, dp);
                        dp.insert(slice, sub_score);
                        score += sub_score;
                    }
                    if score > max {
                        max = score;
                    }
                }
                max
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dailies::d20_12_13::Solution;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let foo = Solution::max_coins(vec![8,3,4,3,5,0,5,6,6,2,8,5,6,2,3,8,3,5,1,0,2]);
    }

    #[test]
    fn test_map_array() {
        let mut map:HashMap<Vec<i32>,bool> = HashMap::new();
        let mut vec = vec![1,2];
        map.insert(vec, true);
        assert_eq!(map[&vec![1,2]], true);
        //assert_eq!(map[vec], true);
    }
}

// 1 -> 1            n  => n
// 32 -> 6 + 3 = 9   xy => x*y + max(x,y)
// 723 ->            xyz => x*y*z + dp[x,z]
// 992 -> 178 + 18 + 9 || 18 + 89 + 9 || 89 + 18 + 9