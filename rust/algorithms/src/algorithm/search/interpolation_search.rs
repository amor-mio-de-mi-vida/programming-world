fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() { return false; }

    let mut low = 0usize;
    let mut high = nums.len() - 1;
    loop {
        let low_val = nums[low];
        let high_cal = nums[high];

        if high <= low || target < low_val || target > high_val {
            break;
        }

        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;

        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    target == nums[high]
}