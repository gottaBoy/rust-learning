mod file;
use num::complex::Complex;

struct Struct {
    e: i32
}

fn main() {
    println!("Hello, world!");
    person_parse();

    complex_sum();

    assert_eq!(ret_unit_type(), ());
}

fn person_parse() {
    let person_data = "\
    name,length(cm)
    first,18
    second,20
    third,10
    Invalid,data
    ";

    let records = person_data.lines();  
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
           continue;
        }
        let fields: Vec<_> = record
        .split(',')
        .map(|field | field.trim()) 
        .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn complex_sum () {
   let a = Complex { re: 2.1, im: -1.2 };
   let b = Complex::new(11.1, 22.2);
   let result = a + b;

   println!("{} + {}i", result.re, result.im);

   let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    let guess = "42".parse::<i32>().expect("Not a number!");
    print!("guess: {}", guess);

    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}, {}", c, z, g, heart_eyed_cat);
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }


    let mut aa = "hello";
    aa = "world";
    println!("aa= {}", aa);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!`

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行 表达式如果不返回任何值，会隐式地返回一个 ()
    let z = if x % 2 == 1 { "odd" } else { "even" };
}


fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
      //...
    };
}


fn arr_test() {
    // 编译器自动推导出one的类型
    let one             = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3]    = [1, 2, 3];
    let blank1          = [0; 3];
    let blank2: [u8; 3] = [0; 3];
  
    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
  
    // 借用arrays的元素用作循环中
    for a in &arrays {
      print!("{:?}: ", a);
      // 将a变成一个迭代器，用于循环
      // 你也可以直接用for n in a {}来进行循环
      for n in a.iter() {
        print!("\t{} + 10 = {}", n, n+10);
      }
  
      let mut sum = 0;
      // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
      for i in 0..a.len() {
        sum += a[i];
      }
      println!("\t({:?} = {})", a, sum);
    }
}
