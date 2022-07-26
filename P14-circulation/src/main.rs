fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // 效率低(第次执行都需进行条件判断)，且取索引容易出错
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // 效率高(取第次执行无需进行条件判断)，索引不会出错
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Range是标准库提供的。当指定一个开始数字和一个结束数字，Range可以生成它们之间的数字（不含结束）
    // rev 方法可以反转Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LITFOFF!!!");
}
