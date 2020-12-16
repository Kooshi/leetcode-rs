

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]),vec![0,1,9,16,100])
    }

    // use test::Bencher;
    // #[bench]
    // fn bench(b: &mut Bencher) {
    //     b.iter(|| Solution::sorted_squares(vec![-4, -1, 0, 3, 10]))
    // }
}
struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.iter().map(|n|n.pow(2)).collect::<Vec<i32>>();
        nums.sort();
        nums
    }
}
