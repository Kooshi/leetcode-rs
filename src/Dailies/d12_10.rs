struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut increase = true;
        let mut increased = false;
        let mut previous = i32::min_value();
        for int in arr {
            if int == previous {
                return false;
            }
            if increase {
                if int < previous {
                    increase = false;
                } else if previous >= 0 {
                    increased = true;
                }
            } else if int > previous {
                return false;
            }
            previous = int;
        }
        increased && !increase
    }
}