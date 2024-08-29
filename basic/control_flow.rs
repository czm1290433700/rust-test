// 控制流：条件&循环
fn main() {
    // if 常规示例
    // if true {
    //     println!("1");
    // } else {
    //     println!("2");
    // }

    // const A: i32 = 1;
    // if A == 1 {
    //     println!("1");
    // } else if A == 2 {
    //     println!("2");
    // } else {
    //     println!("3");
    // }

    // 值得一提的是，rust不会像JavaScript那样自动将非布尔值转换为布尔值，所以在if中使用非布尔值时，必须显式转换为布尔值
    // 例如
    // let number = 3;
    // 这样会报错
    // if number {
    //     println!("number is not zero");
    // }

    // rust中的if有返回值，所以if赋值可以采用以下写法
    // let number = if condition { 5 } else { 6 };
    // println!("The number is: {}", number);
    // 不过rust是强类型语言，所以if的返回值必须是相同类型，所以这里的5和6必须是相同类型
    // let number = if condition { 5 } else { "six" }; // 报错

    // rust中的循环包含loop、while和for三种

    // loop持续执行，直到遇到break，使用break不仅可以中断循环，还可以返回一个值，或者使用continue退出也是可以的，不同的是，continue只退出当次循环
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);

    // while可以在循环中评估某个条件
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
    }

    // for循环
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // 跨两步索引输出
    for number in (1..5).step_by(2) {
        println!("{}", number);
    }
}
