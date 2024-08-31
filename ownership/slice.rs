// 切片引用
// 切片引用是引用的一部分，它允许我们引用String的一部分而不是整个字符串。
// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{}, {}", hello, world);
// }

// 切片首尾的0和len是可选的，如果不写，则默认是0和len - 1
// let s = String::from("hello");

// let slice = &s[0..2];
// let slice = &s[..2];

// let slice = &s[3..len];
// let slice = &s[3..];

// let slice = &s[0..len];
// let slice = &s[..];

// 除字符串，数组同样也可以采取切片
// let a = [1, 2, 3, 4, 5];

// let slice = &a[1..3];

// assert_eq!(slice, &[2, 3]);
