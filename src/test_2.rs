// 相互转换摄氏与华氏温度。
// 生成 n 阶斐波那契数列。
fn main() {
    println!("{}", fahrenheit_to_celsius(0.0));
    println!("{}", celsius_to_fahrenheit(32.0));
    println!("{}",fibonacci(0));
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    32.0 + c * 1.8
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    else if n==2{
        return 2;
    }
    else {
        return fibonacci(n-1)+fibonacci(n-2);
    }
}
