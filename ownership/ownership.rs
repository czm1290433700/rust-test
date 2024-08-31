// rust内存通过所有权系统管理，编译器检查一组规则，如果违反了任何一条规则，程序都无法编译。
// 所有权系统事 rust 独有的新概念，掌握程度决定了能否开发出安全高效的系统。

// 当一个变量超出作用域时，Rust会自动调用drop函数，并为该变量清理堆内存。但针对复制的场景，同时释放相同的内存会导致双重释放错误。
// 所以rust默认永远不会创建堆数据的副本，而是直接转移所有权，或者称为移动。

// String与字符串的区别在于String是可变的，而字符串是不可变的，String使用堆进行存储，而字符串使用栈进行存储
// let mut s1 = String::from("hello");
// s1.push_str(", world!");
// let s2 = s1;

// 报错，因为s1的所有权已被转移给s2，s1无法再使用
// println!("{s1}, world!");

// 如果一定要复制堆数据，可以使用clone方法
// let s1 = String::from("hello");
// let s2 = s1.clone();
// println!("s1 = {s1}, s2 = {s2}");

// 对于堆栈数据，直接复制即可，因为堆栈数据是不可变的，不像堆会有内存释放问题
// let x = 5;
// let y = x;
// println!("x = {x}, y = {y}");

// 如果需要将堆数据作为函数参数传递，则需要使用引用，引用是不可变的。
// fn main() {
//     let s1 = String::from("hello");
//     let len = get_length(&s1);
//     println!("The length of '{s1}' is {len}.");
// }

// 变量s有效的作用域与任何函数参数的作用域相同，但是当s停止使用时，引用所指向的值不会被删除，因为s没有所有权，只是借用。
// fn get_length(s: &String) -> usize {
//     s.len()
// }

// 可变引用加上mut
// fn main() {
//     let mut s = String::from("hello");
//     change_str(&mut s);
//     println!("{}", s);
// }

// fn change_str(s: &mut String) {
//     s.push_str(", world!")
// }

// 同一作用域里堆数据的可变引用只能有一个，而堆数据的不可变引用可以有多个，因为堆数据的不可变引用不会影响堆数据的所有权。
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;
// 报错，因为s的可变引用只能有一个
// println!("{}, {}", r1, r2);

// 悬空引用
// 在Rust中，引用的生命周期是与其关联的作用域紧密相关的。当一个引用（无论是不可变引用&T还是可变引用&mut T）离开其定义的作用域时，该引用就不再有效，并且不能再次使用。
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
// s是在dangle内部创建的，所以当dangle的代码完成时，s将被释放。但是我们试图返回一个对它的引用。这意味着这个引用将指向一个无效的String。
// 可以考虑直接返回堆数据，进行所有权转移，而不是引用
// fn no_dangle() -> String {
//   let s = String::from("hello");
//   s
// }
