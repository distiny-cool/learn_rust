fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let x = 88;
    println!("The value of x is: {x}");
    //下面的不合法，因为x已经被覆盖为不可变变量
    // x = 6;
    // println!("The value of x is: {x}");

    let abc: u8;

    abc = b'n';

    println!("{}", abc);

    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{}", z);

    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; //解构元组为三个变量
    println!("The value of y is: {y}");
    //也可以用.后跟值的索引来直接访问对应元素
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{:X},{},{}", five_hundred,six_point_four,one);

    another_function(x);
    about_expressions();
    about_loop();
    loop_label();
    for_loop();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn about_expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn about_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;	//跳出循环，并返回counter*2的值
        }	//loop语句可以作为let的右侧表达式
    };

    println!("The result is {result}");
    println!("The counter is {counter}");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

//for循环
fn for_loop() {
    //for循环可以很安全的遍历
    //(1..4)是一个range它是标准库提供的类型
    //用来生成从一个数字开始到另一个数字之前结束的所有数字的序列
    //rev()方法用于反转range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}