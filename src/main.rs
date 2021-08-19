/// 变量
/// rust 是强类型语言并且默认数据不可变
/// 可变数据需要加上 mut, 得考虑精度问题
fn variable() {
    let a = 123;
    let mut a = 123;
    a = 456;
    // 不可变变量和常量不同, 不可变变量可以重影, 但是常量的值不可以重影 (标识符重名)
    // 重影是指用同一个标识符代表另一个变量实体
}

/// 数据类型
fn data_type() {
    // 8, 16, 32, 64, 128 有符号, 无符号 int
    // arch, isize, usize 位长度取决于处理器架构
    // 十进制 98_222、十六进制 0xff、八进制 0o77、二进制 0b1111_0000、字节 char (只能表示u8) b'A'
    // f32, f64
    // bool
    // 复合类型
    // `()` 表示不同类型数据元组
    // `[]` 表示同类型数据数组
}

/// 文档注释
/*
* 多行注释
*/
fn comment() {
    // 单行注释
    println!("此为注释");
}

/// fn <标识符>(<参数>) <函数体>
fn function() {
    // 表达式
    let y = {
        let x = 3;
        x + 1
    };

    // 有返回值函数
    fn five() -> i32 {
        5
    }

    // 函数表达式不同于函数体, 不能 return
}

/// 条件语句
fn if_statement() {
    let number = 3;
    if number < 5 {
        println!("true")
    }
    else if number > 0 {
        println!("1212")
    }
    else {
        println!("false")
    }

    // 语句里可以使用函数表达式
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("{}", number);
}

/// loop
fn loop_statement() {
    // while loop
    // for loop -> 迭代器遍历
    let a = [10, 20, 30];
    for i in 0..3 {
        println!("a[{}] = {}", i, a[i])
    }

    for i in a.iter() {
        println!("{}", i)
    }

    // loop loop 无限循环特性
    let a = ["A", "B", "C"];
    let mut i = 0;
    let location = loop {
        let ch = a[i];
        if ch == "B" {
            break i;
        }
        i += 1;
    };
    println!("{}", location)
}

/// rust的所有权机制
/// 所有权规则
/// - Rust 中的每个值都有一个变量，称为其所有者。
/// - 一次只能有一个所有者。
/// - 当所有者不在程序运行范围时，该值将被删除。
/// 数据为堆栈结构
/// 基本数据类型 int、bool、float、char、仅包含上述类型的 Tuples
/// 堆数据有赋值失效的问题, 不定长数据
fn heap_example() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}, World", s1);  // × s1已经失效！
}

/// 克隆
fn heap_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
}

fn main() {
    println!("你好, 世界!");
    println!("{}", "这是我第一个rust程序");
}
