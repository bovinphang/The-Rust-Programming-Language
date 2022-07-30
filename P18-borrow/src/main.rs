fn main() {
    // 不可变引用，示例：
    let s1 = String::from("hello one");
    let len = calculate_length(&s1); // 把引用作为函数参数这个行为叫作借用
    println!("The length of {} is {}", s1, len);

    // 可变引用，示例：
    let mut s2 = String::from("hello two");
    let len2 = calculate_length_2(&mut s2);
    println!("The length of {} is {}", s2, len2);

    // 可变引用有一个重要的限制，在特定作用域内，对某一块数据，只能有一个可变的引用，这样做的好处是可在编译时防止数据竞争，示例：
    let mut s = String::from("hello three");
    let ss1 = &mut s;
    /*
    let ss2 = &mut s; // 报错
    println!("The length of {} is {}", ss1, ss2);
    */
    /*
    引申:
    以下三种行为下会发生数据竞争：
      - 两个或多个指针同时访问一个数据
      - 至少有一个指针用于写入数据
      - 没有使用任何机制来同步对数据的访问
     */

    // 可以通过创建新作用域，来允许非同时的创建多个可变引用，示例：
    let mut s = String::from("hello four");
    {
        let mut ss3 = &mut s;
    }
    let ss4 = &mut s;

    /*
    另一个限制：
      - 不可以同时拥有一个可变引用和一个不变的引用
      - 多个不变的引用是可以的
     */

    let mut s = String::from("hello five");
    let r1 = &s;
    let r2 = &s;
    // let s1 = &mut s; // 报错，不可以将s借用为可变引用，因为它已经借用为不可变引用
    // println!("{} {} {}", r1, r2, s1);

    /*
     悬空引用(Dangling References)
     悬空指针(Dangling Pointer)：一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其它人使用了。
     在Rust里，编译器可保证引用永远都不是悬空引用：
       - 如果你引用了某些数据，编译器将保证在引用离开作用域之前数据不会离开作用域。
       示例：
    */
    let r = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); // 报错，不可以修改借用的东西，其和变量一样，引用默认也是不可变的
    s.len()
}

fn calculate_length_2(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn dangle() -> &String {
    // 编译报错
    let s = String::from("hello");
    &s
}

/*
 总结
 引用的规则
   1. 在任何给定的时刻，只能满足下列条件之一：
     - 一个可变的引用
     - 任意数据不可变的引用
   2. 引用必须一直有效
*/
