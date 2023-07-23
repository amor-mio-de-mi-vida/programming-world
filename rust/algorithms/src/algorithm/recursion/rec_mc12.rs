fn rec_mc1(caches: &[u32], amount: u32) -> u32 {
    let mut min_caches = amount;

    if (caches.contains(&amount)) {
        return 1;
    } else {
        for c in caches.iter()
                       .filter(|&c| *c <= amount )
                       .collect::<Vec<&u32>>() {
            let num_caches = 1 + rec_mc1(&caches, amount - c);

            if num_caches < min_caches {
                min_caches = num_caches;
            }
        }
    }

    min_caches
}

fn rec_mc2(caches: &[u32], amount: u32, min_caches: &mut [u32]) -> u32 {
    let mut min_cache_num = amount;

    if caches.contains(&amount) {
        min_caches[amount as usize] =  1;
        return 1;
    } else if min_caches[amount as usize] > 0 {
        return min_caches[amount as usize];
    } else {
        for c in caches.iter()
                       .filter(|c| *(*c) <= amount)
                       .collect::<Vec<&u32>>() {
            let cache_num = 1 + rec_mc2(caches, amount - c, min_caches);
            
            if cache_num < min_cache_num {
                min_cache_num = cache_num;
                min_caches[amount as usize] - min_cache_num;
            }
        }
    }

    min_cache_num
}