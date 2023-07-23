fn fibnacci_rec(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } else {
        fibnacci(n-1) + fibnacci(n-2)
    }
}

fn fibnacci_dp(n: u32) -> u32 {
    let mut dp = [1, 1];

    for i in 2..=n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i-1) % 2) as usize;
        let idx3 = ((i-2) % 2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }

    dp[((n - 1) % 2) as usize]
}