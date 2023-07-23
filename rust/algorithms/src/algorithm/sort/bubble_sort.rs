fn bubble_sort1(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j+1] {
                nums.swap(j, j+1);
            }
        }
    }
}

fn bubble_sort2(nums: &mut [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
            }
        }
        len -= 1;
    }
}

fn bubble_sort3(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
                compare = true;
            }
        }
        len -= 1;
    }
}

fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;
            // 从左到右冒泡
            for j in i..(len - i - 1) {
                if nums[j] > nums[j+1] {
                    nums.swap(j, j+1);
                    bubble = true;
                }
            }
            // 从右到左冒泡
            for j in (i+1..=(len - i - 1)).rev() {
                if nums[j] < nums[j-1] {
                    nums.swap(j-1, j);
                    bubble = true;
                }
            } 
        } else {
                break;
            }
    }
}

fn comb_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    let mut i;
    let mut gap: usize = nums.len();

    // 大致排序， 数据基本有序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i-gap] > nums[i] {
                nums.swap(i-gap, i);
            }
            i += 1;
        }
    }

    // 细致调节部分无序数据， exchange控制是否继续交换数据
    let mut exchange = true;
    // 这个cnt是什么？
    while exchange {
        exchange = false;
        i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
                exchange = true;
            }
            i += 1;
        }
    }
}

fn cbic_sort1(nums: &mut[i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

fn cbic_sort2(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }

    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    comb_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}