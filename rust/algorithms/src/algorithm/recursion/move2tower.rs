fn move2tower(height: u32, src_p: &str, des_p: &str, mid_p: &str) {
    if height >= 1 {
        move2tower(height - 1, src_p, mid_p, des_p);
        println!("moving dist from {src_p} to {dst_p}");
        move2tower(height - 1, mid_p, des_p, src_p);
    }
}