const BASESTR: [&str; 16] = ["0","0","0","0","0","0","0","0",
                            "0","0","0","0","0","0","0"];

fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        // 余数加在末尾
        num2str_rec(num/base, base) + BASESTR[(num % base) as usize]
    }
}

fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = ["0","1","2","3","4","5","6","7",
    "8","9","A","B","C","D","E","F"];
     
    let mut rem_stack = Stack::new();
    while num > 0 {
        if num < base {
        rem_stack.push(num); // 不超过 base 直接入栈
        } else { // 超过 base 余数入栈
        rem_stack.push(num % base);
        }
        num /= base;
        }
        
        // 出 栈 余 数 并 组 成 字 符 串
        let mut numstr = "".to_string();
        while !rem_stack.is_empty() {
        numstr += digits[rem_stack.pop().unwrap() as usize];
    }
        
    numstr
}