use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io; //引入io库
fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0..100);
    //println!("The secret num is:{n}");

    let mut guess = String::new();
    /*let用于创建变量（默认不可变）
    加入mut后则创建可变变量
    这里创建可变变量guess，绑定的值为String的新的空实例 */

    loop {
        println!("Input your guess:");

        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        /*调用io库中的函数stdin
        再调用read_line方法从标准输入句柄中获得用户输入，并追加到变量guess中
        &mut guess表示是guess的可变引用
        readline会返回一个result类型（一种枚举类型）的值
        expect为Result实例的一种方法，如果实例值为Err，expect会导致程序崩溃，并显示当做参数传递给 expect 的信息 */

        println!("You guessed : {guess}");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /* Rust 允许用一个新值来隐藏（shadow）guess之前的值。
        这个功能常用在需要转换值类型之类的场景。
        它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量 */
        /*trim 方法会去除字符串开头和结尾的空白字符
        parse 方法用于将字符串转换为其他类型 */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /*将expect调用换成match语句，忽略非数字的猜测并重新请求数字而不是让程序崩溃 */

        match guess.cmp(&n) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
