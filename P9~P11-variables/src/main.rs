const MAX_POINTS: u32 = 100_000;
fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("Hello, world!");

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    // let mut spaces = "    ";
    // spaces = spaces.len();
    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 54 % 5;

    let t = true;
    let f: bool = false;

    let x = 'z';
    let y: char = 'Ƶ';
    let z = '😂';

    // 声明元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 元组解构赋值
    let (x, y, z) = tup;
    println!("{}, {}, {} ", x, y, z);
    println!("{}, {}, {} ", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first = months[0];
    let second = months[1];

    // 数组类型的表示: [类型; 长度]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 另一种声明数组的方法
    let a = [3; 5]; // 相当于：let a = [3, 3, 3, 3, 3];

    // 访问数组的索引超出数组的范围
    // 简单索引情况，编译会报错，运行也会报错
    //let index = 15;
    //let month = months[index];
    // 复杂索引情况，编译会通过，运行会报错
    let index = [12, 13, 14, 15];
    let month = months[index[1]];
    println!("{}", month);
}
