fn quick_sort1(nums: &mut [i32], low: usize, high:usize) {
    if low  < high {
        let split = partition(nums, low, high);
        if split > 1 {
            quick_sort1(nums, low, split - 1);
        }
        quick_sort1(nums, split + 1, high);
    }
}

fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;
    let mut rm = high;
    loop {
        // 左标记不断右移
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        } 
        // 左表记越过右标记时退出并交换左右标记数据
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }
    nums.swap(low, rm);

    rm
}

fn quick_sort2(nums: &mut [i32], low: usize, high: usize) {
    if low >= high { return; }

    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        // 右标记不断左移
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }
        // 左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 交换左右标记处数据
        nums.swap(lm, rm);
    }
    // 交换分割点数据
    nums.swap(low, lm);

    if lm > 1 {
        quick_sort2(nums, low, lm - 1);
    }

    quick_sort2(nums, rm + 1, high);
}

fn main() {
    let mut nums = [54,26,93,17,77,31,44,55,20];
    let len = nums.len();
    quick_sort1(&mut nums, 0, len - 1);
    println!("sorted nums: {:?}", nums);

    let mut nums = [54,26,93,17,77,31,44,55,20];
    let len = nums.len();
    quick_sort2(&mut nums, 0, len - 1);
    println!("sorted nums: {:?}", nums);
}