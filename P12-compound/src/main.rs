fn main() {
    another_function(5, 6); // argument 调用函数时的实参

    let y = 6; // 语句无返回值
               // let x = (let y = 6); // 错误的写法，语句不可以赋值给一个变量。在c和ruby则是正确的。

    let x = 5;
    let y = {
        let x = 1;
        x + 3
        // x + 3; // 加分号则变成了语句，相当于： ()
    };
    println!("The value of y is： {}", y);

    let x = plus_five(6);
    println!("The value of x is： {}", x);
}

fn another_function(x: i32, y: i32) {
    // parameter 定义函数参数时的形参
    println!("The value of x is： {}", x);
    println!("The value of y is： {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
