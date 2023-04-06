/*
fn main() {
    /*  基本输出
    let a = 12;
    println!("a is {}", a);
    println!("a is {0},a again is {0}",a);  //可变参数当成数组,下标从0开始
    println!("{{}}");   //{用来转义{, }转义}
    */

    //let a = 123;
    //a = "abc";    已声明a是整型
    //a = 4.56;     自动转换,数据精度有损失
    //a = 456;      a不是可变变量

    /*  重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。
        但可变变量赋值仅能发生值的变化。
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x)    //The value of x is: 12
    */
    //let mut s = "123";
    //s = s.len();  不能给字符串变量赋整型值

    /*  整型
    let a: i8 = 0o77;    //8-bit    八进制
    let b: i16 = 0xff;   //16-bit   十六进制
    let c = 1_23;  //32-bit    十进制  下划线为了容易判断数字的大概值
    let d: i64 = 0b1111_0000;   //64-bit    二进制
    let e: i128 = 123456;  //128-bit
    let f: isize = 12345678; //arch-取决于目标平台处理器架构位数
    let g: u8 = b'A';   //字节,只能表示u8型
    */

    //  浮点型
    //let x = 1.23;   //f64
    //let y: f32 = 4.56;   //f32

    /* rust不支持++和--,因为这两个运算符出现在变量前后影响代码可读性 */

    //元组用一对()表示的一组数据,可以包含不同类型
    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    //数组用一对[]表示的同类型数据
    let mut a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("a[0] is {}", first);
    a[0] = 0;
    println!("Now, a[0] is {}", a[0]);
}
*/

/*
/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
///
/// ```

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn main() {
    println!("{}", add(2, 3));
    print();
}

fn print() {
    println!("Hello World!");
}
*/
fn main() {
    let a = 12;
    let b;
    if a > 0 {  //rust中条件表达式必须是bool类型
        b = 1;
    } else {
        b = 0;
    }
    println!("b is {}", b);
}