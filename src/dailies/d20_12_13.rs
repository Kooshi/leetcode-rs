

struct Solution;

//problem 312
use std::collections::HashMap;
use std::iter::FromIterator;

struct Balloons<'a>{
    nums: &'a[i32],
    remaining: Vec<usize>
}
impl<'a> Balloons<'a> {
    fn new(nums: &'a [i32]) -> Self {
        Self {
            nums,
            remaining: Vec::from_iter(0..nums.len()),
        }
    }

    fn pop(&self, i: usize) -> (Self, i32) {
        let mut remaining = self.remaining.clone();
        let i = remaining.remove(i);
        let left = remaining.iter().filter(|v| **v < i).max().map(|l| self.nums[*l]).unwrap_or(1);
        let right = remaining.iter().filter(|v| **v > i).min().map(|l| self.nums[*l]).unwrap_or(1);

        (Self {
            nums: self.nums,
            remaining
        },
         left * self.nums[i] * right
        )
    }

    fn score(&self) -> i32 {
        self.score_recurse(&mut HashMap::new())
    }

    fn score_recurse(&self, dp: &mut HashMap<Vec<i32>, i32>) -> i32 {
        match self.remaining.len() {
            0 => 0,
            1 => self.nums[self.remaining[0]],
            2 => self.nums[self.remaining[0]] * self.nums[self.remaining[1]]
                + self.nums[self.remaining[0]].max(self.nums[self.remaining[1]]),
            _ => {
                let mut max = 0;
                for i in 0..self.remaining.len() {
                    let (new_b, mut score) = self.pop(i);
                    let rem: Vec<i32> = new_b.remaining.iter().map(|i| self.nums[*i]).collect();
                    if dp.contains_key(&rem) {
                        score += dp[&rem];
                    } else {
                        let sub_score = new_b.score_recurse(dp);
                        dp.insert(rem, sub_score);
                        score += sub_score;
                    }

                    if score > max {
                        max = score
                    }
                }
                max
            }
        }
    }
}

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        Balloons::new(&nums).score()
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