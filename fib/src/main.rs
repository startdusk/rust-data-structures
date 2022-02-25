fn main() {
    for i in 0..10 {
        println!(
            "naive={}, iter={}, dynamic={}, dynamic_tail_recursice={}",
            fibonacci(i),
            fibonacci_iter(i),
            fibonacci_dynamic(i).0,
            fibonacci_dynamic_tail_recursice(i, 1, 1)
        )
    }
}

// 原生递归斐波那契数列
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

// 循环斐波那契数列
pub fn fibonacci_iter(n: i32) -> i32 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let tmp_b = b;
        b = a + b;
        a = tmp_b;
    }

    b
}

// 动态规划斐波那契数列
// return (res, prev)
// If you are going to use the same function more than once,
// store the result somewhere
pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    if n == 0 {
        return (1, 0);
    }

    let (a, b) = fibonacci_dynamic(n - 1);
    (a + b, a)
}

// 斐波那契数列使用尾递归实现
// 如果一个函数返回自身递归调用的结果，那么调用过程会被替换为一个循环，它可以显著提高速度。
// 尾递归是一种在函数的最后执行递归调用语句的特殊形式的递归
// 尾递归就是从最后开始计算, 每递归一次就算出相应的结果, 也就是说, 函数调用出现在调用者函数的尾部, 因为是尾部, 所以根本没有必要去保存任何局部变量。
// 直接让被调用的函数返回时越过调用者,返回到调用者的调用者去。
pub fn fibonacci_dynamic_tail_recursice(n: i32, prev: i32, res: i32) -> i32 {
    if n == 0 {
        return prev;
    }

    fibonacci_dynamic_tail_recursice(n - 1, res, prev + res)
}
