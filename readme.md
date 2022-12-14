# 尽快学会Rust，冲冲冲！！！

#### 代码 ./src/程序名_对应官方文档章节号.rs

官方文档：https://doc.rust-lang.org/stable/book/

中文译本：https://kaisery.github.io/trpl-zh-cn/title-page.html

参考书籍：深入浅出Rust

RUST是一个系统编程语言，有三大特点：runs blazingly fast, prevents segfaults, and guarantees thread safety.

### 第一部分 基础知识

项目：https://github.com/rust-lang/rust

相关讨论：https://github.com/rust-lang/rfcs

Rust语言更新步骤：RFC——Nightly——Beta——Stable

标准库文档：https://www.rust-lang.org/learn

文档：rustup docs --book

```
 C:\Users\24426\.cargo\bin 的目录

2022/07/30  11:39    <DIR>          .
2022/07/30  11:39    <DIR>          ..
2022/07/30  11:31        10,077,696 cargo-clippy.exe
2022/07/30  11:31        10,077,696 cargo-fmt.exe			//源代码格式化工具
2022/07/30  11:31        10,077,696 cargo-miri.exe
2022/07/30  11:31        10,077,696 cargo.exe				//包管理器
2022/07/30  11:31        10,077,696 clippy-driver.exe
2022/07/30  11:31        10,077,696 rls.exe					//代码提示工具
2022/07/30  11:31        10,077,696 rust-gdb.exe
2022/07/30  11:31        10,077,696 rust-gdbgui.exe
2022/07/30  11:31        10,077,696 rust-lldb.exe
2022/07/30  11:31        10,077,696 rustc.exe				//编译器
2022/07/30  11:31        10,077,696 rustdoc.exe				//文档生成器
2022/07/30  11:31        10,077,696 rustfmt.exe				//源代码格式化工具
2022/07/30  11:31        10,077,696 rustup.exe				//更新工具
```

中科大代理：https://mirrors.ustc.edu.cn/help/crates.io-index.html

Windows 下对应的设置环境变量的 PowerShell 命令为：

```
$env:RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
$env:RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"
```

helloworld.rs 

```rust
//helloworld.rs
fn main(){
  let s = "hello world!";
  println!("{}",s);
}
```

注意，其中的println！是一个宏，，而不是一个函数，是因为标准输入宏可用完成编译期格式检查，即如果出现参数个数、格式等各种原因不匹配会直接导致编译错误，更加安全。

Rust 是一种 **预编译静态类型**（*ahead-of-time compiled*）语言，类似于C

> 编译型语言：先编译，再运行，区别于解释型语言（python）
>
> 静态类型语言：运行时结构不可变的语言，区别于动态语言（JS，PY）
>
> 强类型语言，一旦一个变量被指定了某个数据类型，如果不经过强制类型转换，那么它就永远是这个数据类型，区别于弱类型语言（JS）

#### 项目编写与调试

rust的生态不错，编写项目和调试都比较方便。

环境：Vscode+插件（rust-analyzer;CodeLLDB)

方法：

```
$ cargo new hello_cargo
$ cd hello_cargo
$ code ./
# 进入vscode后选择.rs文件，直接点击Start Debugging，根据提示自动创建json文件，即可正常调试
```

#### 编写程序：猜数字游戏

程序代码：src/guess_num.rs

:question:一开始loop没有包括let mut guess = String::new(); 结果发现guess的值一直没有改变，而且显示也好像有问题。

:crossed_fingers:结果发现，程序并没有出错，而是`io::stdin().read_line(&mut guess)`是将用户输入附加到传递给它的字符串中，所以后面输入得到的其实是[数字]1\n[数字2]

总结：

1. rust的概念了解：let、match、函数、外部crate

2. *crate* 是一个 Rust 代码包，调用外部crate可以通过修改*Cargo.toml* 文件

   例如：在 *Cargo.toml* 文件中[dependencies] 片段后添加 `rand = "0.8.3"`，引入rand依赖

   注意：`0.8.3` 事实上是 `^0.8.3` 的简写，它表示任何至少是 `0.8.3` 但小于 `0.9.0` 的版本。

3. `cargo doc --open`可以用来构建所有本地依赖提供的文档

#### 常见编程概念

1. **不可变变量和常量的区别**

   - 不允许对常量使用 `mut`，常量不光默认不能变，它总是不能变
   - 声明常量使用 `const` 关键字而不是 `let`，并且 *必须* 注明值的类型
   - 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值

   > rust默认变量为不可变（immutable），这样的好处是增加了安全性，而且代码更易于推导

2. **隐藏**（Shadowing）

   > 隐藏是指类似于`let x = 5;  let x = x + 1;`这样的操作，此时，第二个x“遮蔽”了第一个x，时任何使用该变量名的行为中都会视为是在使用第二个变量，直到第二个变量自己也被隐藏或第二个变量的作用域结束。可以用相同变量名称来隐藏一个变量，以及重复使用 `let` 关键字来多次隐藏。

   当再次使用 `let` 时，实际上创建了一个新变量，我们可以改变值的类型（包括设置为可变变量或者不可变），并且复用这个名字。

3. **标量（*scalar*）类型**代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

   :warning:在debug模式编译时，Rust 检查整形溢出的问题并使程序 *panic*，这个术语被 Rust 用来表明程序因错误而退出。而在release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（*two’s complement wrapping*）的操作。简而言之，值 `256` 变成 `0`，值 `257` 变成 `1`，依此类推。

   :warning:整数除法会向下舍入到最接近的整数。

4. Rust中用单引号声明 `char` 字面量，而与之相反的是，使用双引号声明字符串字面量。Rust 的 `char` 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 `char` 值。

   ```rust
   fn main() {
       let c = 'z';
       let z: char = 'ℤ'; // with explicit type annotation
       let heart_eyed_cat = '😻';
   }
   ```

5. **复合类型**（*Compound types*）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。

   **元组**：元组长度固定，一旦声明，其长度不会增大或缩小。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的

   ```rust
   fn main() {
       let tup: (i32, f64, u8) = (500, 6.4, 1);
       let (x, y, z) = tup;	//解构元组为三个变量
       println!("The value of y is: {y}");
       //也可以用.后跟值的索引来直接访问对应元素
       let five_hundred = tup.0;
       let six_point_four = tup.1;
       let one = tup.2;
   }
   ```

   > 不带任何值的元组有个特殊的名称，叫做 **单元（unit）** 元组。这种值以及对应的类型都写作 `()`，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。

   **数组**：与元组不同，数组中的每个元素的类型必须相同。Rust 中的数组与一些其他语言中的数组不同，Rust中的数组长度是固定的。

   ```rust
   let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
   
   let a: [i32; 5] = [1, 2, 3, 4, 5];
   //i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素
   
   let b = [3; 5];
   //变量名为b的数组将包含5个元素，这些元素的值最初都将被设置为3。
   
   let one = a[0];
   let four = a[3];
   ```

   当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。如果索引超出了数组长度，Rust 会 panic。

   > panic是 Rust 术语，它用于程序因为错误而退出的情况。在很多底层语言中，并没有进行这类检查，这样当提供了一个不正确的索引时，就会访问无效的内存。通过立即退出而不是允许内存访问并继续执行，Rust 让你避开此类错误。

6. **函数**：

   Rust 代码中的函数和变量名使用 *snake case* 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。

   Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。

   在函数定义中，**必须** 声明每个参数的类型。

   ```rust
   fn main() {
       print_labeled_measurement(5, 'h');
   }
   
   fn print_labeled_measurement(value: i32, unit_label: char) {
       println!("The measurement is: {value}{unit_label}");
   }
   ```

7. **语句和表达式**

   **语句**（*Statements*）是执行一些操作但不返回值的指令。

   **表达式**（*Expressions*）计算并产生一个值。

   Rust 是一门基于表达式（expression-based）的语言

   `let x = (let y = 6);`语句是非法的，因为在Rust中`let y = 6` 语句并不返回值。

   ```rust
   # 注意 x + 1后面没有分号
   let y = {
       let x = 3;
       x + 1
   };
   println!("The value of y is: {y}");
   ```

   :exclamation:`{let x = 3; x + 1}`是一个代码块，它的值是 `4`。这个值作为 `let` 语句的一部分被绑定到 `y` 上。注意 `x+1` 这一行在结尾没有分号，与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。

8. 函数的返回值

     在 Rust 中，函数头部并不对返回值命名，但要在箭头（`->`）后声明它的类型。函数的返回值等同于函数体最后一个表达式的值。使用 `return` 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。

   ```rust
   fn main() {
       let x = plus_one(5);
   
       println!("The value of x is: {x}");
   }
   
   fn plus_one(x: i32) -> i32 {
       // x + 1;
       //如果加了分号，则变成了语句，并不会返回值，与函数定义返回i32的值相矛盾，出现错误。
       x + 1
   }
   ```

9. **if语句**中的条件 **必须** 是 `bool` 值。如果条件不是 `bool` 值，我们将得到一个错误。

   ```rust
   fn main() {
       let number = 3;
       
       if number < 5 {
           println!("condition was true");
       } else {
           println!("condition was false");
       }
   }
   ```

   :warning: if语句可以在let语句的右侧作为表达式，但是需要if和else分支的结果是同一类型，如果类型不匹配则会报错！

   > 因为变量必须只有一个类型，而且Rust 需要在编译时就确切的知道变量的类型，这样它就可以在编译时验证在每处使用的变量类型是有效的。如果变量类型仅在运行时确定，则 Rust 无法做到这一点；且编译器必须跟踪每一个变量的多种假设类型，那么它就会变得更加复杂，对代码的保证也会减少。

10. Rust 有三种**循环**：`loop`、`while` 和 `for`。

    ```rust
    //loop循环
    fn main() {
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
    ```

    如果存在嵌套循环，`break` 和 `continue` 应用于此时最内层的循环。你可以选择在一个循环上指定一个 **循环标签**（*loop label*），然后将标签与 `break` 或 `continue` 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。

    ```rust
    //for循环
    fn main() {
        //for循环可以很安全的遍历
        //(1..4)是一个range它是标准库提供的类型
        //用来生成从一个数字开始到另一个数字之前结束的所有数字的序列
        //rev()方法用于反转range
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    ```

### 第二部分 内存安全

#### 所有权

> 所有权规则：
>
> 1. Rust 中的每一个值都有一个被称为其 **所有者**（*owner*）的变量。
> 2. 值在任一时刻有且只有一个所有者。
> 3. 当所有者（变量）离开作用域，这个值将被丢弃。

以下代码不可以运行

```rust
fn main() {
    let mut s1 = String::from("hello");
    let s2 = s1; 	//已经将s1的堆上数据移动给了s2，s1失效
    println!("{}", s1);  //发生错误
}
```

如果要把复制堆上面的数据，可以用clone函数：`let s2 = s1.clone();`

> 如果数据仅仅在栈上，就不用考虑是移动还是拷贝了

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域
    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 drop方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
```

Rust 有一个叫做 `Copy` trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。如果一个类型实现了 `Copy` trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 `Drop` trait 的类型使用 `Copy` trait。

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 转移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将
                                             // 返回值移动给
                                             // 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域.
    some_string                              // 返回 some_string 
                                             // 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```

为了不至于出现总是将一个变量传入函数后改变所有权，导致函数结束后原来的变量不再可用，Rust使用了引用的方法。

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {	//s是String类型变量的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
```

:warning: Rust中的引用不同于C++中的引用，默认其不允许修改引用的值。如果想要使得引用的值可用改变，需要在函数参数中添加mut。

```rust
fn main() {
    let mut s = String::from("hello");
    //change(&s);
    change(&mut s);
}

//fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

> 可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用。
>
> 这是因为Rust会避免**数据竞争**的发生，不会编译存在数据竞争的代码！
>
> 下面是同一作用域中两类引用的共存情况。

|            | 不可变引用 | 可变引用 |
| :--------: | :--------: | :------: |
| 不可变引用 |   可共存   | 不可共存 |
|  可变引用  |  不可共存  | 不可共存 |

:warning:Rust中不存在无效的引用（类似于C中的悬垂指针），下列代码不能编译成功

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！编译器报错！
```

```rust
//获取字符串第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    //下面的语句出错
    //因为这里的word是slice类型，属于s的不可变引用，所以不能再出现s的可变引用。
    let a = &mut s;
    println!("the first word is: {}", word);
}
```

字符串字面值就是 slice，例如`let s = "Hello, world!";`这里的s的类型就是`&str`:它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；`&str` 是一个不可变引用。

#### 结构体

#### 包和crate

#### 错误处理

