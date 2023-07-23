fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1;
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i;
            }
        }

        nums.swap(left, pos_max);
        left -= 1;
    }
}