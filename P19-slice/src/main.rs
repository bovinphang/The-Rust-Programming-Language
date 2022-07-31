fn main() {
    let mut s = String::from("Hello world!");
    let word_index = first_word(&s);
    // s.clear();
    println!("{}", word_index);

    let word = first_word2(&s);
    println!("{}", word);
    /*
     Rust的另外一种不持有所有权的数据类型：切片(slice)
     字符串切片是指向字符串中一部分内容的引用，形式：[开始索引..结束索引]
       - 开始索引就是切片起始位置的索引值
       - 结束索引就是切片终止位置的下一个索引值
    示例：
    */
    let s = String::from("Hello world!");
    let hello = &s[0..5];
    // 或：
    // let hello = &s[..5];
    let world = &s[6..11];
    // 或：
    // let world = &s[6..s.len()];
    // 或：
    // let world = &s[6..];
    println!("{}, {}!", hello, world);

    // let whole = &s[0..s.len()];
    // 或：
    let whole = &s[..];
    println!("{}", whole);

    /*
      注意：
        1. 字符串切片的范围必须发生在有效的UTF-8字符边界内。
        2. 如果尝试从一个多字节的字符中创建字符串切片，程序会报错并退出。
    */

    /*
     字符串字面值是切片
       1. 字符串字面值被直接存储在二进制程序中。
       2. 比如：let s = "Hello World!";
       3. 变量s的类型是 &str, 它是一个指向二进制程序特定位置的切片
            - &str 是不可变引用，所以字符串变量值也是不可变的
    */

    /*
     将字符串切片作为参数传递
       1. fn first_word(s: &String) -> &str {
       2. 有经验的Rust开发者会采用 &str 作为参数类型，因为这样就可以同时接收 String 和 &str 类型的参数了：
       3. fn first_word(s: &str) -> &str {
           - 使用字符串切片，直接调用该函数
           - 使用String, 可用创建一个完整的 String 切片来调用该函数
       4. 定义函数时使用字符串切片来代替字符串引用会使我们的 API 更加通用，且不会损失任何功能
       示例：
    */
    let my_string = String::from("Hello World");
    let word = first_word3(&my_string[..]);
    let my_string_literal = "Hello World";
    let word = first_word3(my_string_literal);

    // 其它类型的切片：
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.trim().as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 用切片重写：
fn first_word2(s: &String) -> &str {
    let bytes = s.trim().as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.trim().as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
