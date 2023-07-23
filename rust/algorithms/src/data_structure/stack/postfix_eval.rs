fn postfix_eval(postfix: &str) -> Option<i32> {
    // 少于5个字符，不是有效的后缀表达式，因为表达式至少两个操作数加一个
    // 操作符，还需要两个空格隔开。
    if postfix.len() < 5 { return None; }

    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if "0" <= token && token <= "9" {
            op_stack.push(token.parse::<i32>.unwrap());
        } else {
            // 对于减法和除法， 顺序有要求
            // 所以先出栈的是第二个操作数
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            let res = do_calc(token, op1, op2);
            op_stack.push(res);
        }
    }
    Some(op_stack.pop().unwrap())
}

fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    }
}