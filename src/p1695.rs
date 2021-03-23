
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!( 15,Solution::maximum_unique_subarray(vec![5,1,2,3,4,5]));
        assert_eq!(16911, Solution::maximum_unique_subarray(vec![187,470,25,436,538,809,441,167,477,110,275,133,666,345,411,459,490,266,987,965,429,166,809,340,467,318,125,165,809,610,31,585,970,306,42,189,169,743,78,810,70,382,367,490,787,670,476,278,775,673,299,19,893,817,971,458,409,886,434]))
    }

    // use test::Bencher;
    // #[bench]
    // fn bench(b: &mut Bencher) {
    //     b.iter(|| )
    // }
}
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut set = HashSet::with_capacity(nums.len());

        let mut l = 0;
        let mut r = 1;
        set.insert(nums[l]);
        let mut sum = nums[l];
        let mut max_sum = sum;
        while r < nums.len() {
            while r < nums.len() && !set.contains(&nums[r]) {
                set.insert(nums[r]);
                sum += nums[r];
                r += 1;
            }
            if sum > max_sum {
                max_sum = sum;
            }
            while r < nums.len() && set.contains(&nums[r]) {
                set.remove(&nums[l]);
                sum -= nums[l];
                l += 1;
            }
        }
        max_sum
    }
}