fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }

    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }
    let low = high >> 1;

    binary_search(&nums[low..size.min(high+1)], target)
}