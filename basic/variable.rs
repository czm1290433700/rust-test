// 变量，常量
fn main() {
    // 变量声明后不可修改
    // let x = 5;
    // x = 6;
    // println!("The value of x is: {}", x);

    // 变量声明后可修改
    // let mut y = 5;
    // println!("The value of y is: {}", y);
    // y = 6;
    // println!("The value of y is: {}", y);

    // 不使用mut的let虽然默认不可变，但可以通过后续加上mut改变为可变变量
    // let z = 5;
    // let mut z = z;
    // z = z + 1;
    // println!("The value of z is: {}", z);

    // 使用const定义的是常量，总不可变
    // const z: i32 = 5;
    // let mut z = z;
    // z = z + 1;
    // println!("The value of z is: {}", z);

    // 同名变量类型覆写，并不强迫使用不同变量名，后定义的会覆盖之前的定义
    // let x = '1';
    // println!("The value of x is: {}", x);
    // let x: i32 = 0;
    // println!("The value of x is: {}", x);
}
