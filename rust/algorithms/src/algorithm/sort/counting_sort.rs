fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 { return; }

    // 桶数量为nums 中最大值加1，保证数据都有桶放
    let max_bkt_num = num.iter().max.unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] + 1;
    }

    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0  {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}