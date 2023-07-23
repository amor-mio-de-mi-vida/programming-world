fn dp_rec_mc_show(caches: &[u32], amount: u32, min_caches: &mut [u32], caches_used: &mut [u32]) -> u32 {
    for denm in 1..=amount {
        let mut min_cache_num = denm;
        let mut used_cache = 1; //最小面额为1元
        for c in caches.iter()
                       .filter( |&c| * c <= denm )
                       .collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cache_num = 1 + min_caches[index];
            if cache_num < min_cache_num {
                min_cache_num = cache_num;
                used_cache = *c;
            }            
        }

        min_caches[denm as usize] = min_cache_num;
        caches_used[denm as usize] = used_cache;
    }
    min_caches[amount as usize]
} 