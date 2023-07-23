# Rust快速入门

## Rust旅程

- 编译 helloworld.rs 文件

> ps: `rustc helloworld.rs`
>
> ps: `rustc helloworld.rs -O` # 也可以选择优化编译

- 运行程序

> ps: `./helloworld.exe` # windows 平台下需要加.exe后缀

- 创建项目 hellorust

> ps: `cargo new hellorust --bin`

- 查看目录结构

> ps :`tree`

- 查看Cargo.toml文件

> ps: `cat Cargo.toml`

```toml
name = "hellorust"
version = "0.1."
authors = ["Your name"]
[dependencies]
```

- 编辑src目录下的main.rs文件

> ps: `subl ./src/main.rs`

编译和运行

> ps: `cargo build`
>
> ps: `cargo build –-release` # 这个属于优化编译
>
> ps: `./target/debug/hellorust.exe`
>
> ps: `./target/release/hellorust.exe` # 如果前面是优化编译，则这样运行
>
> ps: `cargo run` # 编译和运行合在一起
>
> ps: `cargo run –-release` # 同上，区别是优化编译的



## 变量绑定与原生类型

Rust内置的原生类型 (primitive types) 有以下几类：

- 布尔类型：有两个值`true`和`false`。
- 字符类型：表示单个Unicode字符，存储为4个字节。
- 数值类型：分为有符号整数 (`i8`, `i16`, `i32`, `i64`, `isize`)、 无符号整数 (`u8`, `u16`, `u32`, `u64`, `usize`) 以及浮点数 (`f32`, `f64`)。
- 字符串类型：最底层的是不定长类型`str`，更常用的是字符串切片`&str`和堆分配字符串`String`， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
- 数组：具有固定大小，并且元素都是同种类型，可表示为`[T; N]`。
- 切片：引用一个数组的部分数据并且不需要拷贝，可表示为`&[T]`。
- 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
- 指针：最底层的是裸指针`*const T`和`*mut T`，但解引用它们是不安全的，必须放到`unsafe`块里。
- 函数：具有函数类型的变量实质上是一个函数指针。
- 元类型：即`()`，其唯一的值也是`()`

```rust
// boolean type
let t = true;
let f: bool = false;

// char type
let c = 'c';

// numeric types
let x = 42;
let y: u32 = 123_456;
let z: f64 = 1.23e+2;
let zero = z.abs_sub(123.4);
let bin = 0b1111_0000;
let oct = 0o7320_1546;
let hex = 0xf23a_b049;

// string types
let str = "Hello, world!";
let mut string = str.to_string();

// arrays and slices
let a = [0, 1, 2, 3, 4];
let middle = &a[1..4];
let mut ten_zeros: [i64; 10] = [0; 10];

// tuples
let tuple: (i32, &str) = (50, "hello");
let (fifty, _) = tuple;
let hello = tuple.1;

// raw pointers
let x = 5;
let raw = &x as *const i32;
let points_at = unsafe { *raw };

// functions
fn foo(x: i32) -> i32 { x }
let bar: fn(i32) -> i32 = foo;
```

有几点是需要特别注意的：

- 数值类型可以使用`_`分隔符来增加可读性。
- Rust还支持单字节字符`b'H'`以及单字节字符串`b"Hello"`，仅限制于ASCII字符。 此外，还可以使用`r#"..."#`标记来表示原始字符串，不需要对特殊字符进行转义。
- 使用`&`符号将`String`类型转换成`&str`类型很廉价， 但是使用`to_string()`方法将`&str`转换到`String`类型涉及到分配内存， 除非很有必要否则不要这么做。
- 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏`vec!`创建。
- 元组可以使用`==`和`!=`运算符来判断是否相同。
- 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
- Rust不提供原生类型之间的隐式转换，只能使用`as`关键字显式转换。
- 可以使用`type`关键字定义某个类型的别名，并且应该采用驼峰命名法。

```rust
// explicit conversion
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

// type aliases
type NanoSecond = u64;
type Point = (u8, u8);
```

## 数组、动态数组和字符串

`str` 类型基本上不怎么使用，通常使用 `&str` 类型，它其实是 `[u8]` 类型的切片形式 `&[u8]`。这是一种固定大小的字符串类型。 常见的的字符串字面值就是 `&'static str` 类型。这是一种带有 `'static` 生命周期的 &str 类型。

**例子：**

```rust
// 字符串字面值
let hello = "Hello, world!";

// 附带显式类型标识
let hello: &'static str = "Hello, world!";
```

`String` 是一个带有的 `vec:Vec<u8>` 成员的结构体，你可以理解为 `str` 类型的动态形式。 它们的关系相当于 `[T]` 和 `Vec<T>` 的关系。 显然 `String` 类型也有压入和弹出。

**例子：**

```rust
// 创建一个空的字符串
let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
hello.push('w');
hello.push_str("orld!");

// 弹出字符。
let mut s = String::from("foo");
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));
assert_eq!(s.pop(), None);
```



## 结构体与枚举

```rust
// structs
struct Point {
  x: i32,
  y: i32,
}
let point = Point { x: 0, y: 0 };

// tuple structs
struct Color(u8, u8, u8);
let android_green = Color(0xa4, 0xc6, 0x39);
let Color(red, green, blue) = android_green;

// A tuple struct’s constructors can be used as functions.
struct Digit(i32);
let v = vec![0, 1, 2];
let d: Vec<Digit> = v.into_iter().map(Digit).collect();

// newtype: a tuple struct with only one element
struct Inches(i32);
let length = Inches(10);
let Inches(integer_length) = length;

// unit-like structs
struct EmptyStruct;
let empty = EmptyStruct;
```

需要注意，Rust在语言级别不支持域可变性 (field mutability)，所以不能这么写：

```rust
struct Point {
    mut x: i32,
    y: i32,
}
```

此外，结构体的域对其所在模块 (mod) 之外默认是私有的，可以使用`pub`关键字将其设置成公开。

**枚举**

Rust有一个集合类型，称为枚举 (enum)，代表一系列子数据类型的集合。 其中子数据结构可以为空-如果全部子数据结构都是空的，就等价于C语言中的enum。 我们需要使用`::`来获得每个元素的名称。

```rust
// enums
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

let x: Message = Message::Move { x: 3, y: 4 };
```

与结构体一样，枚举中的元素默认不能使用关系运算符进行比较 (如`==`, `!=`, `>=`)， 也不支持像`+`和`*`这样的双目运算符，需要自己实现，或者使用`match`进行匹配。



**控制流**

Match

```rust
let day = 5

match day {
	0 | 6 => println!("weekend"),
	1 ... 5 => println!("weekday"),
	_ => println!("invalid"),
}
```

`_`在这里是必须的，因为`match` 强调进行穷尽性检查，必须覆盖所有可能值。如果需要得到 `|` 或者`…`匹配到的值，可以使用`@`绑定变量：

```rust
let x = 1;

match x {
	e @ 1 ... 5 => println!("got a range element {}", e),
	_ => println!("anything"),
}
```

使用`ref`关键字来得到一个引用。

```rust
let x = 5;
let mut y = 5;

match x {
	// the 'r' inside the match has the type `&i32`
	ref r => println!("Got a reference to {}", r),
}

match y {
	// the 'mr' inside the match has the type `&i32` and is mutable
	ref mut mr => println!("Got a mutable reference to {}", mr),
}
```

再看一个使用`match` 表达式来解构元组的例子：

```rust
let pair = (0, -2);

match pair {
	(0, y) => println!("x is `0` and `y` is `{:?}`", y),
	(x, 0) => println!("`x` is `{:?}` and y is `0`", x),
	_ => println!("It doesn't matter what they are"),	
}
```

match的这种解构同样适用于结构体或者枚举。如果有必要， 还可以使用`..`来忽略域或者数据。

```rust
struct Point {
	x: i32,
	y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin{
	Point { x, .. } => println!("x is {}", x),
}

enum OptionalInt {
	Value(i32),
	Missing,
}

let x = OptionalInt::Value(5);

match x {
	// 这里是match的if guard 表达式
	OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
	OptionalInt::Value(..) => println!("Got an int!"),
	OptionalInt::Missing => println!("No such luck."),
}
```

此外，Rust还引入了`if let`和`while let`进行模式匹配。

```rust
let number = Some(7);
let mut optional = Some(0);

// If `let` destructures `numeber` into `Some(i)`, evaluate the block.
if let Some(i) = number {
	println!("Matched {:?}!", i);
} else {
	println!("Didn't match a number!");
}

// While `let` destructuers `optional` into `Some(i)`, evaluate the block.
while let Some(i) = optional {
	if i > 9 {
		println!("Greater than 9, quit!");
		optional = None;
	} else {
		println!("`i` is `{:?}`. Try again.", i);
		optional = Some(i + 1);
	}
}
```



## 函数与方法

要声明一个函数，需要使用关键字`fn`, 后面跟上函数名，比如

```rust
fn add_one(x: i32) -> i32 {
	x + 1
}
```

其中函数参数的类型不能省略，可以有多个参数，但是最多只能返回一个值。可以使用属性`#[allow(dead_code)]` 禁用无效代码检查。

Rust有一个特殊特性适用于发散函数(diverging function)， 它不返回

```rust
fn diverges() -> {
	panic!("This function never returns!");
}
```

其中`panic!`是一个宏， 使当前执行线程崩溃并打印指定信息。返回类型 `!` 可用作任何类型

```rust
let x: i32 = diverges();
let y: String = diverges();
```

匿名函数

Rust使用闭包(closure)来创建匿名函数：

```rust
let num = 5;
let plus_num = |x: i32| x + num
```

其中闭包`plus_num` 借用了它作用域中的`let` 绑定`num`。如果要让闭包获得所有权，可以使用`move`关键字。

```rust
let mut num = 5;

{
	let mut add_num = move |x: i32| num += x;	// 闭包通过move获取了num的所有权
	
	add_num(5);
}

// 下面的num在被move之后还能继续使用是因为其实现了Copy特性
// 具体可见所有权(Ownership)章节
assert_eq!(5, num);
```

Rust还支持高阶函数， 允许把闭包作为参数来生成新的函数。

```rust
fn add_one(x: i32) -> i32 { x+1 }

fn apply<F>(f: F, y: i32) -> i32
	where F: Fn(i32) ->i32
{
	f(y) * y
}

fn factory(x: i32) -> Box<Fn(i32) -> i32> {
	Box::new(move |y| x + y)
}

fn main() {
	let transform: fn(i32) -> i32 = add_one;
	let f0 = add_one(2i32) * 2;
	let f1 = apply(add_one, 2);
	let f2 = apply(transform, 2);
	println!("{}, {}, {}", f0, f1, f2);
	
	let closure = |x: i32| x + 1;
	let c0 = closure(2i32) * 2;
	let c1 = apply(closure, 2);
	let c2 = apply(|x| x + 1, 2);
	println!("{}, {}, {}", c0, c1, c2);
	
	let box_fn = factory(1i32);
	let b0 = box_fn(2i32) * 2;
	let b1 = (*box_fn)(2i32) * 2;
	let b2 = (&box_fn)(2i32) * 2;
	println!("{}, {}, {}", b0, b1, b2);
	
	let add_num = &(*box_fn);
	let translate: &Fn(i32) -> i32 = add_num;
	let z0 = add_num(2i32) * 2;
	let z1 = apply(add_num, 2);
	let z2 = apply(translate, 2);
	println1("{}, {}, {}, z0, z1, z2");
}
```

