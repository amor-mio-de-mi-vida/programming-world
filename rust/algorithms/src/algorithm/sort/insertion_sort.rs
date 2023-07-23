fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos-1] {
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }
        nums[pos] = curr;
    }
}

fn bin_insertion_sort(nums: &mut [i32]) {
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len() {
        left = 0;
        right = i - 1;
        temp = nums[i];

        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                // 防止出现 right = 0 - 1
                if 0 == mid { break; }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        // 将数据后移， 留出空位
        for j in (left..=i-1).rev() {
            nums.swap(j, j+1);
        }
        // 将temp插入空位
        if left != i {
            nums[left] = temp;
        }
    }
}

fn main() {
    let mut nums = [54,32,99,18,75,31,43,56,21];
    insertion_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}