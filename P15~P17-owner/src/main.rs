fn main() {
    // s不可用
    let s = "hello"; // s可用，s是字符串字面值，不可修改（不可面性）
                     // 可以对 s 进行相关操作

    let mut st = String::from("Hello"); // “::”表示from是String类型下的函数
    st.push_str(", World");
    println!("{}", st);

    let x = 5; // 整数5绑定到变量x中
    let y = x; // 创建一个x的副本，并这个副本绑定到变量y中
               // 整数是已知且固定大小的简单的值，这两个5被压到了stack中
    println!("{}, {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // 当把s1赋值给s2，String的数据被复制了一份：1、在stack上复制了一份指针、长度、容量；2、并没复制指针所指向的heap上的数据。

    // println!("{}", s1); // 报错，s1失效了，因为它的所有权已移交给s2

    let s11 = String::from("hello");
    let s22 = s11.clone();
    println!("{}, {}", s11, s22);

    let ss = String::from("Hello, World!!!");
    take_ownership(ss);
    // println!("ss: {}", ss); // 报错，ss失效了，因为它的所有权已移交给take_ownership这个函数里面了

    let x = 5;
    makes_copy(x);
    println!("x: {}", x);

    let ss1 = gives_ownership();
    let ss2 = String::from("hello");
    let ss3 = takes_and_gives_back(ss2);

    let ss4 = String::from("hello");
    let (ss5, len) = calculate_length(ss4);

    println!("The length of {} is {}", ss5, len);
} // s 作用域到此结束，s 不再可用

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
