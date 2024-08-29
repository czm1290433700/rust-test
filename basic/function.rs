// rust 函数命名建议使用小写字母+下划线定义
fn main() {
    // rust 不关心你在哪里定义函数，只关心它们在调用者可以看到的作用域中的某个地方定义。
    // 所以即使 another_fn 定义在 main 后，仍然不影响在 main 中调用 another_fn
    // another_fn();

    // another_fn(5);

    // let return_val = another_fn(5);
    // println!("{}", return_val);
}

// 常规函数定义
// fn another_fn() {
//     println!("callback another_fn");
// }

// 带参数的函数定义，在函数签名中，必须声明每个参数的类型
// fn another_fn(x: i32) {
//     println!("x is {}", x);
// }

// 带返回值的函数定义
fn another_fn(x: i32) -> i32 {
    // rust支持隐式返回内容，而不需要使用return，不过这种隐式返回不能加;，否则它就是一个语句而不是隐式返回了
    // x

    // 当然使用return返回也是可以的
    // return x;
}
