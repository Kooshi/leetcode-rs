pub struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        //does not need to be this big, but whatever, it works, first dp done
        let mut dp = vec![vec![i32::min_value(); stones.len()]; stones.len()];
        for off in 0..stones.len() {
            for l in 0..stones.len() {
                if l + off == stones.len() {
                    break;
                } else if l == 0 && l + off == stones.len() - 1 {
                    return Solution::stone_game_score(l, l + off, &stones, &dp);
                }
                dp[l][l + off] = Solution::stone_game_score(l, l + off, &stones, &dp);
            }
        }
        0
    }

    fn stone_game_score(l: usize, r: usize, stones: &Vec<i32>, dp: &Vec<Vec<i32>>) -> i32 {
        match r - l + 1 {
            1 => 0,
            2 => stones[l].max(stones[r]),
            _ => {
                (stones[l..r].iter().sum::<i32>() - dp[l][r - 1])
                    .max(stones[l + 1..r + 1].iter().sum::<i32>() - dp[l + 1][r])
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(6, p1690::Solution::stone_game_vii(vec![5,3,1,4,2]));
        assert_eq!(122, p1690::Solution::stone_game_vii(vec![7,90,5,1,100,10,10,2]));
    }
}