use std::marker::PhantomData;
use std::hash::{BuildHasher, Hash, Hasher};
use std::colections::hash_map::{DefaultHasher, RandomState};

// 布隆过滤器
struct BloomFilter<T: ?Sized> {
    bits: Vec<bool>,  //比特桶
    hash_fn_count: usize,   // 哈希函数个数
    hashers: [DefaultHasher; 2],    // 两个哈希函数
    _phantom: PhantomData<T>,   // T 占位， 欺骗编译器
}

impl<T: ?Sized + Hash> BloomFilter<T> {
    fn new(cap: usize, ert: f64) -> Self {
        let ln22 = std::f64::consts::LN_2.powf(2f64);
        // 计算比特桶大小和哈希函数个数
        let bits_count = -1f64 * cap as f64 * ert.ln() / ln22;
        let hash_fn_count = -1f64 * ert.log2();

        // 随机哈希函数
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        Self {
            bits: vec![false; bits_count.ceil() as usize],
            hash_fn_count: hash_cn_count.ceil() as usize,
            hashers: hashers,
            _phantom: PhantomData,
        }
    }

    // 按照 hash_fn_count 计算值并置比特桶相应位为true
    fn insert(&mut self, elem: &T) {
        let hashes = self.make_hash(elem);
        for fn_i in 0..self.hash_fn_count {
            let index = self.get_index(hashes, fn_i as u64);
            self.bits[index] = true;
        }
    }

    // 数据查询
    fn contains(&self, elem: &T) -> bool {
        let hashes = self.make_hash(elem);
        (0..self.hash_fn_count).all(|fn_i| {
            let index = self.get_index(hashes, fn_i as u64);
            self.bits[index]
        })
    }

    
}