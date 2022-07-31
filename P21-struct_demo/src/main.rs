/*
  struct例子：计算长方形的面积
*/
/*
fn main() {
    let w = 30;
    let h = 50;
    println!("{}", area(w, h));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
// 缺点：长和宽无关联
*/

// 优化一：
/*
fn main() {
    let rect = (30, 50);
    println!("{}", area(rect));
}
fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}
// 缺点，元组没有名字，容易搞混，分不出哪个是长哪个是宽
*/

// 优化二：
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", area(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
}
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

/*
 总结：
   - 什么是struct
   - std::fmt::Display
   - std::fmt::Debug
   - #[derive(Debug)]
   - {:?}
   - {:#?}
*/
