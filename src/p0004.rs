struct Solution;



impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let mut half = (total / 2) + 1;
        let even = total % 2 == 0;
        if even {
            half -= 1;
        }
        let mut count = 0;
        let mut x1 = 0;
        let mut x2 = 0;
        let mut previous = 0;
        let mut current = 0;
        if nums1.is_empty() && nums2.len() == 1 {
            return nums2[0] as f64;
        } else if nums2.is_empty() && nums1.len() == 1 {
            return nums1[0] as f64;
        }
        while count <= half {
            previous = current;
            if x1 == nums1.len() {
                current = nums2[x2];
                x2 += 1;
            } else if x2 == nums2.len() {
                current = nums1[x1];
                x1 += 1;
            } else if nums1[x1] < nums2[x2] {
                current = nums1[x1];
                x1 += 1;
            } else {
                current = nums2[x2];
                x2 += 1;
            }
            count += 1;
        }
        if even {
            (current as f64 + previous as f64) / 2 as f64
        } else {
            previous as f64
        }
    }
}