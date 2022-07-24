use rand::Rng;
use std::cmp::Ordering;
use std::io; // prelude //trait

fn main() {
    println!("猜数游戏!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // i32 u32 i64

    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数!");
        // let mut foo = 2;
        // let bar = foo; // immutable
        // foo = 2;

        let mut guess = String::new();

        // std::io::stdin().read_line(&mut guess).expect("无法读取行");
        io::stdin().read_line(&mut guess).expect("无法读取行");
        // io::Result Ok,Err

        // shadow
        // let guess: u32 = guess.trim().parse().expect("Please type a number!"); // trim()也会把\n过滤

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测试的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"), // arm
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
