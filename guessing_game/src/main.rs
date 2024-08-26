// 入口文件
// cargo build构建为二进制可执行文件
// cargo run构建为二进制可执行文件并运行
use rand::Rng;
use std::io;

fn main() {
    println!("猜字游戏开始！");
    let secret_number = rand::thread_rng().gen_range(1..10);
    loop {
        let mut guess = String::new();
        println!("请输入你的猜测:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // rust支持重用guess变量名，而不是强迫必须重新声明一个变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入字符非法，请输入一个数字");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("你猜的字小了"),
            std::cmp::Ordering::Greater => println!("你猜的字大了"),
            std::cmp::Ordering::Equal => {
                println!("你猜对了！");
                break;
            }
        }
    }
}
