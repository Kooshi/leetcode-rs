struct Solution;


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut deduped_i = 0;
        let mut previous = i32::min_value();
        let mut one_dup = false;
        for i in 0..nums.len() {
            let val = nums[i];
            if val != previous || !one_dup {
                if deduped_i < i {
                    nums[deduped_i] = val;
                }
                deduped_i += 1;

                if val != previous {
                    one_dup = false;
                } else {
                    one_dup = true;
                }
            }
            previous = nums[i];
        }
        nums.truncate(deduped_i);
        nums.len() as i32
    }
}