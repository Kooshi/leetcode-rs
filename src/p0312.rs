

struct Solution;

//problem 312
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {

    }
}

#[cfg(test)]
mod tests {
    use crate::dailies::d20_12_13::Solution;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        //36s
        let foo = Solution::max_coins(vec![8,3,4,3,5,0,5,6,6,2,8,5,6,2,3,8,3,5,1,0,2]);
    }

    #[test]
    fn long() {
        assert_eq!( Solution::max_coins(vec![5,7,8,1]), 336);
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