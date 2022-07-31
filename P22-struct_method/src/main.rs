/*
  struct的方法：
    1. 方法和函数类似：fn 关键字、名称、参数、返回值
    2. 方法与函数不同之处：
        - 方法是在struct（或enum、trait对象）的上下文中定义
        - 第一个参数是 self，表示方法被调用的struct实例

  定义方法：
    1. 在impl块里定义方法
    2. 方法的第一个参数是 &self，也可以获得其所有权: self 或 可变借用: &mut self，就像其它参数一样。
    3. 更良好的代码组织。

  方法调用的运算符：
    1. C/C++：object->something() 和 (*object).something() 一样。
    2. Rust 没有->运算符
    3. Rust 会自动引用或解引用
        - 在调用方法时就会发生这种行为
    4. 在调用方法时，Rust 根据情况自动添加 &、&mut 或 *（解引用符号），以便 object 可以匹配方法的签名。
    5. 下面两行代码效果相同：
        - p1.distance(&p2);
        - (&p1).distance(&p2);

  方法参数：
    方法可以有多个参数。


  关联函数
    1. 可以在 impl 块里定义不把 self 作为第一个参数的函数，它们叫关联函数（不是方法），不是在实例上面调用的
        - 例如：String::from()
    2. 关联函数通常用于构造器
    3. :: 符号
        - 关联函数
        - 模块创建的命名空间

  多个 impl 块
    每个 struct 允许拥有多个 impl 块

*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 定义 Rectangle 的方法：
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法可以有多个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    // 正方形
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 每个 struct 允许拥有多个 impl 块
impl Rectangle {
    fn test(&self) -> &str {
        "abc"
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };
    println!("{}", rect1.area());
    // 或：
    println!("{}", (&rect1).area());
    println!("{:?}", rect1);
    println!("{:#?}", rect1);

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let s = Rectangle::square(20);
    println!("{:#?}", s);
}
