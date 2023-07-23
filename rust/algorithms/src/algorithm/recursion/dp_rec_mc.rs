fn dp_rec_mc(caches: &[u32], amount: u32, min_caches: &mut [u32]) -> u32 {
    for denm in 1..=amount{
        let mut min_cache_num = denm;
        for c in caches.iter()
                       .filter(|&c| *c <= denm )
                       .collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;

            let cache_num = 1 + min_caches[index];
            if cache_num < min_cache_num {
                min_cache_num = cache_num;
            }
        } 
        min_caches[denm as usize] = min_cache_num;
    }

    min_caches[amount as usize]
}