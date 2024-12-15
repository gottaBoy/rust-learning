# 目录
```rust
cargo new project
cargo new mylib --lib
cargo doc --open

cargo run -- needle haystack
cargo run -- the test.txt
cargo test
cargo run -- -q to -p test.txt -i true

Rust 提供的解决方案是通过命令行参数来控制:
RUST_BACKTRACE=1 cargo run
与之类似，我们也可以使用环境变量来控制大小写敏感，例如:
IGNORE_CASE=1 cargo run -- to poem.txt
```

## Rust 的标准目录结构
唯一库包：src/lib.rs
默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
集成测试文件：tests 目录下
基准性能测试 benchmark 文件：benches 目录下
项目示例：examples 目录下

# Rust 基本概念
所有权、借用、生命周期
宏编程
模式匹配

## 变量绑定与解构
变量命名
变量绑定
变量可变性
变量解构
变量遮蔽(shadowing)

## 基本类型
数值类型, 字符、布尔、单元类型

语句和表达式
函数

## 所有权和借用
### 所有权
- 垃圾回收机制(GC)，在程序运行时不断寻找不再使用的内存，典型代表：Java、Go
- 手动管理内存的分配和释放, 在程序中，通过函数调用的方式来申请和释放内存，典型代表：C++
- 通过所有权来管理内存，编译器在编译时会根据一系列规则进行检查

悬空指针(Dangling Pointer)
栈(Stack)与堆(Heap)

#### 所有权原则
- Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
- 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
- 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)

#### 变量绑定背后的数据交互
转移所有权
实际上， String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成，其中堆指针是最重要的，它指向了真实存储字符串内容的堆内存，至于长度和容量，如果你有 Go 语言的经验，这里就很好理解：容量是堆内存分配空间的大小，长度是目前已经使用的大小。

Rust 这样解决问题：当 s1 被赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了

浅拷贝(shallow copy) 和 深拷贝(deep copy)，那么拷贝指针、长度和容量而不拷贝数据听起来就像浅拷贝
克隆(深拷贝)
首先，Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

拷贝(浅拷贝)
浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);

```

任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

所有整数类型，比如 u32
布尔类型，bool，它的值是 true 和 false
所有浮点数类型，比如 f64
字符类型，char
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意：可变引用 &mut T 是不可以 Copy的

### 引用与借用

#### 引用与解引用
#### 不可变引用
通过 &s1 语法，我们创建了一个指向 s1 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域后，其指向的值也不会被丢弃
#### 可变引用
可变引用同时只能存在一个
不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：


两个或更多的指针同时访问同一数据
至少有一个指针被用来写入数据
没有同步数据访问的机制

可变引用与不可变引用不能同时存在

注意，引用的作用域 s 从创建开始，一直持续到它最后一次使用的地方，这个跟变量的作用域有所不同，变量的作用域从创建持续到某一个花括号 }

#### 悬垂引用(Dangling References)
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

## 复合类型
read 函数也非常有趣，它返回一个 ! 类型，这个表明该函数是一个发散函数，不会返回任何值，包括 ()。unimplemented!() 告诉编译器该函数尚未实现，unimplemented!() 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码，类似的标记还有 todo!()

### 字符串 & 切片(slice)
Rust 的重点也是难点：字符串 String 和 &str
```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。

#### String 与 &str 的转换
```rust
String::from("hello,world")
"hello,world".to_string()

fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}
```
字符串索引
字符串切片
字符串转义
操作 UTF-8 字符串

- 字符
```rust
for c in "中国人".chars() {
    println!("{}", c);
}
```

- 字节
```rust
for b in "中国人".bytes() {
    println!("{}", b);
}
```

#### 操作字符串

### 元组
#### 用模式匹配解构元组
#### 用 . 来访问元组
#### 元组的使用示例

### 结构体
#### 结构体语法
#### 定义结构体
#### 创建结构体实例
#### 访问结构体字段
#### 简化结构体创建
#### 结构体更新语法
#### 结构体的内存排列
#### 元组结构体(Tuple Struct)
#### 单元结构体(Unit-like Struct)
#### 结构体数据的所有权
#### 使用 #[derive(Debug)] 来打印结构体的信息

### 枚举
枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型
#### 同一化类型
#### Option 枚举用于处理空值

### 数组
在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 array，第二种是可动态增长的但是有性能损耗的 Vector，在本书中，我们称 array 为数组，Vector 为动态数组。

- 数组的三要素：
长度固定
元素必须有相同的类型
依次线性排列

数组 array 是存储在栈上，性能也会非常优秀。与此对应，动态数组 Vector 是存储在堆上

- 数组切片
数组类型容易跟数组切片混淆，[T;n] 描述了一个数组的类型，而 [T] 描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用 [T;n] 的形式去描述
[u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小

## 流程控制
### 使用 if 来做分支控制
### 使用 else if 来处理多重条件
### 循环控制
### for 循环
### continue
### break
### while 循环
### while vs for
### loop 循环

## 模式匹配
### match 和 if let
#### match 匹配
##### 使用 match 表达式赋值
##### 模式绑定
##### 模式绑定
##### 穷尽匹配
##### _ 通配符
#### if let 匹配
#### matches!宏
#### 变量遮蔽

### 解构 Option
解构 Option
匹配 Option<T>
传入参数 Some(5)
传入参数 None

### 模式适用场景
#### 模式
模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，它往往和 match 表达式联用，以实现强大的模式匹配能力。模式一般由以下内容组合而成：

字面值
解构的数组、枚举、结构体或者元组
变量
通配符
占位符

#### match 分支
```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    _ => EXPRESSION,
}

```
#### if let 分支
```rust 
if let PATTERN = SOME_VALUE {

}

```
#### while let 条件循环
```rust
// Vec是动态数组
let mut stack = Vec::new();

// 向数组尾部插入元素
stack.push(1);
stack.push(2);
stack.push(3);

// stack.pop从数组尾部弹出元素
while let Some(top) = stack.pop() {
    println!("{}", top);
}

```
#### for 循环
```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}

```
#### let 语句
```rust
let PATTERN = EXPRESSION;
```

#### 函数参数
#### let 和 if let
#### let-else(Rust 1.65 新增)
使用 let-else 匹配，即可使 let 变为可驳模式。它可以使用 else 分支来处理模式不匹配的情况，但是 else 分支中必须用发散的代码块处理（例如：break、return、panic）


### 全模式列表

## 方法 Method
### 定义方法
### self、&self 和 &mut self
self 依然有所有权的概念：

self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
&self 表示该方法对 Rectangle 的不可变借用
&mut self 表示可变借用

使用方法代替函数有以下好处：
不用在函数签名中重复书写 self 对应的类型
代码的组织性和内聚性更强，对于代码维护和阅读来说，好处巨大

方法名跟结构体字段名相同
代码是等价的：
p1.distance(&p2);
(&p1).distance(&p2);

## 泛型和特征
### 泛型 Generics
### 特征 Trait
### 特征对象
### 深入了解特征

## 集合类型
### 动态数组 Vector
### KV 存储 HashMap

## 认识生命周期
### 悬垂指针和生命周期
### 借用检查
### 函数中的生命周期

## 返回值和错误处理
Rust 中的错误主要分为两类：

可恢复错误，通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误，这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
不可恢复错误，刚好相反，该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，这些错误的影响往往对于系统来说是致命的
### panic 深入剖析
#### panic! 与不可恢复错误
#### 主动调用
#### backtrace 栈展开
#### panic 时的两种终止方式
### 可恢复的错误 Result 和 ？

## 包和模块
Rust 也提供了相应概念用于代码的组织管理：

项目(Packages)：一个 Cargo 提供的 feature，可以用来构建、测试和分享包
包(Crate)：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
模块(Module)：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元
包和 Package

项目(Package)：可以用来构建、测试和分享包
工作空间(WorkSpace)：对于大型项目，可以进一步将多个包联合在一起，组织成工作空间
包(Crate)：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
模块(Module)：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元

## 注释和文档
### 代码注释
### 文档注释
Panics：函数可能会出现的异常状况，这样调用函数的人就可以提前规避
Errors：描述可能出现的错误及什么情况会导致错误，有助于调用者针对不同的错误采取不同的处理方式
Safety：如果函数使用 unsafe 代码，那么调用者就需要注意一些使用条件，以确保 unsafe 代码块的正常工作
### 包和模块级别的注释

## 格式化输出
### print!，println!，format!
它们是 Rust 中用来格式化输出的三大金刚，用途如下：

print! 将格式化文本输出到标准输出，不带换行符
println! 同上，但是在行的末尾添加换行符
format! 将格式化文本输出到 String 字符串

eprint!，eprintln!
除了三大金刚外，还有两大护法，使用方式跟 print!，println! 很像，但是它们输出到标准错误输出：

eprintln!("Error: Could not complete task")

与 {} 类似，{:?} 也是占位符：
### {} 与
{} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
{:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景

### Debug 特征
### Display 特征
### 为自定义类型实现 Display 特征
```rust
struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}
fn main() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}
```

### 为外部类型实现 Display 特征
```rust
struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}
fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}
```

rust上手了一段时间感觉还是比较ok的，除了lifetime，lifetime那里有点迷，其它很多都可以从c++和go的视角代入来看，rust的宏简直就是我的神，我go使用的是我自己编写的“宏”来生成go代码的，go的运行时反射我也在用，但还是不仅人意，很多地方明明能在编译期间做的，它就只能在运行时用反射，所以我后来自己搞了个"宏"生成代码，但是新问题又来了，生成的代码必须实际写入go文件，又会造成代码冗杂，后面的其他同事接手代码又会懵逼。而rust的宏是比较合理的，用syn将tokenstream解析为ast语法树，然后用quote将需要生成的代码走模版返回而不是跟胶水一样东瓶西凑


## 文档
- [rust](https://course.rs/basic/ownership/ownership.html)
- [rust-by-practice](https://github.com/sunface/rust-by-practice)
- [字符串](https://course.rs/basic/compound-type/string-slice.html)
- [Rust 秘典（死灵书)](https://nomicon.purewhite.io/vec/vec.html)
- [ahash](https://github.com/tkaitchuck/ahash)
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [num](https://crates.io/crates/num)
- [Rust By Practice( Rust 练习实践 )](https://practice-zh.course.rs/type-conversions/from-into.html)
- [rust-by-example](https://rustwiki.org/zh-CN/rust-by-example/hello.html)
- [too-many-lists](https://rust-unofficial.github.io/too-many-lists/)
- [Rusty Book(锈书)](https://rusty.course.rs/devtools/log.html)
- [rust-learning](https://github.com/gottaBoy/rust-learning)
- [Rust 语言圣经](https://course.rs/about-book.html)
- [Rust 语言圣经(中文版)](https://rustlang.cn/book/title-page.html)
- [Rust 语言圣经(英文版)](https://doc.rust-lang.org/book/title-page.html)
- [BestSRE](https://github.com/mingongge/BestSRE)
- [BestSRE(中文版)](https://github.com/mingongge/BestSRE/blob/master/README.md)
- [Nightingale](https://mp.weixin.qq.com/s/pvfaNKEPd74TuHUE8m5lpQ)
- [CSS—flex布局、常用水平垂直居中](https://juejin.cn/post/6844904098777710599)
- [too-many-lists](https://rust-unofficial.github.io/too-many-lists/third-layout.html)
- [comprehensive-rust](https://google.github.io/comprehensive-rust/)
- [comprehensive-rust(中文版)](https://google.github.io/comprehensive-rust/zh-CN/other-resources.html)
- [rust-algos](https://github.com/rustcn-org/rust-algos)
- [Rust 练习实践](https://practice-zh.course.rs/type-conversions)
- [polonius](https://github.com/rust-lang/polonius)
- [Rusty Book](https://rusty.course.rs/devtools/log.html)

