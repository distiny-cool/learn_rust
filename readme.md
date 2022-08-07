# 尽快学会Rust，冲冲冲！！！

参考书籍：深入浅出Rust

RUST是一个系统编程语言，有三大特点：runs blazingly fast, prevents segfaults, and guarantees thread safety.

### 第一部分 基础知识

#### Chapter 1 

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

