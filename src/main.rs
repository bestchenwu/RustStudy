mod second;

use std::fmt::format;
use std::fs::File;
use std::{fs, io, thread};
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;
use crate::Book::Papery;
use crate::second::ClassName;

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

impl Site {
    //结构体方法,第一个参数是&self
    fn speak(&self) -> String {
        self.name.clone()
    }
    //结构体关联函数,方法定义在结构体内,但是参数没有引用对象
    fn create(domain: String, name: String, nation: String, found: u32) -> Site {
        Site { domain, name, nation, found }
    }
}

#[derive(Debug)]
//枚举类
enum Book {
    Papery,
    Electronic,
    //也可以给枚举对象
    url_book { url: String },
    index_book { index: u32 },
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

trait Descriptive {
    fn describe(&self) -> String {
        String::from("from trait Descriptive")
    }
}

struct Person {
    age: u32,
    name: String,
}

impl Descriptive for Person {
    // fn describe(&self) -> String {
    //     format!("age:{0},name:{1}", self.age, self.name)
    // }
}

fn output<T: Descriptive>(t: T) -> String {
    t.describe()
}

fn main() {
    // let args = std::env::args();
    // println!("{:?}",args)
    // use std::io::stdin;
    // // let mut str = String::new();
    // // stdin().read_to_string(&mut str).expect("failed to read line");
    // // println!("your input:\n{}",str);
    // let str = fs::read_to_string("D:\\newJob.txt").unwrap();
    // println!("str={}",str);
    //let a = 12;
    //println!("a is {0},a again is {0} " ,a )
    //println!("Hello, world!");
    //println!("{{}}")
    //默认是不可变变量
    // let a = 12;
    // //如果要变成可变变量
    // let mut b = 23;
    // b = 456;
    // println!("{0}",b)
    //重影 同一个名字代表另一个变量实体,其类型、可变变量、值都可以发生变化
    // let x = 5;
    // let x = x+1;
    // let x = x*2;
    // //println!("x={}",x);
    // //print!("{}",x);
    // let  x = 98_322;
    // let y = b'A';
    // print!("{}",y);
    // let z:bool = true;
    // let t = (11,22,"abc");
    // let a = [3;5];
    // //等价于
    // let b = [3,3,3,3,3];
    // another_function(32,64);
    //
    // let y = {
    //   let x = 32;
    //   x*4
    // };
    // print!("y={}",y)
    // let number = 3;
    // if number<5 {
    //     println!("number = {},is below 5",number)
    // }else{
    //     println!("number = {},is over 5",number)
    // }
    //
    // let x = if number<5 {1} else {0};
    // println!("x={}",x)
    // let mut number = 1;
    // while number<4 {
    //     println!("number={}",number);
    //     number+=1;
    // }

    //let a = [10, 20, 30, 40, 50];
    //for 循环
    // for b in a.iter() {
    //     println!("b={}", b)
    // }
    //while break循环,在rust里面是loop
    // let mut i = 0;
    // loop {
    //     let x:i64 = a[i];
    //     if x == 30 {
    //         break;
    //     }
    //     println!("x={}", x);
    //     i += 1;
    // }
    //这种方式是创建了一个对象
    //let str1 = String::from("abc");
    //称为数据的移动
    // let str2 = str1;
    // //这时候str1已经被回收了
    // //这个声明 只是创建了一个指针
    // let str3 = "abc";
    // println!("str={}", str1);
    //如果像str1 str2都有效,可以采用克隆的方式
    //let str2 = str1.clone();
    //或者建立str1的引用
    // let str2 = &str1;
    // let str3 = str1;
    // //这个时候会报错,因为str1已经不拥有对象的所有权了,所有权已经移动到了str3
    // //println!("str={}", str2);
    // //如果像继续使用str1,则需要重新引用
    // let str2 = &str3;
    // println!("str={}", str2);
    // let mut str1 = String::from("abc");
    // //这里str2是不可变引用,会提示不允许对不可变引用改变值
    // // let str2 = &str1;
    // // str2.push_str("haha");
    // let str2 = &mut str1;
    // str2.push_str("haha");
    // //这个时候输出abchaha
    // println!("str={}", str2);
    //println!("str1={0},str2={1}", str1, str2);
    // let dangle_reference = dangle();
    // println!("str={}",dangle_reference)
    //字符串切片
    // let str = "broadcast";
    // //表示从0到第5个元素,但不包括第5个元素
    // let str1 = &str[0..5];
    // let str2 = &str[5..9];
    // println!("str1={0},str2={1}",str1,str2);
    // let runob = Site {
    //     domain: String::from("www.baidu.com"),
    //     name: String::from("runob"),
    //     nation: String::from("china"),
    //     found: 2013,
    // };
    // let site0 = Site {
    //     domain: String::from("www.runob.com"),
    //     //表示除了domain外,其他字段都引用runob对象的值
    //     ..runob
    // };
    //println!("site0 domain={0},name={1},nation={2},found={3}", site0.domain, site0.name, site0.nation, site0.found);
    //或者使用rust自带的输出语句
    // println!("site = {:?}", site0);
    // println!("speak : {}", site0.speak())
    //println!("create site : {:?}", Site::create(String::from("www.raoshanshan.com"),String::from("raoshanshan"),String::from("china"),32));
    // let book = Book::Papery;
    // println!("{:?}", book);
    // let index_book = Book::index_book { index: 32 };
    // let url_book = Book::url_book {url:String::from("www.lisisi.com")};
    // // println!("{:?}", index_book);
    // // println!("{:?}", url_book);
    // match url_book {
    //     Book::index_book { index } => {
    //         println!("index:{}", index)
    //     }
    //     Book::url_book { url } => {
    //         println!("url:{}", url)
    //     }
    //     _ => {
    //         println!("book:{:?}", index_book)
    //     }
    // }
    // let some = Some("hello");
    // match some {
    //     Some(something) =>{
    //         println!("something:{}",something);
    //     }
    //     None =>{
    //         println!("nothing");
    //     }
    // }
    // let i = 0;
    // match i {
    //     0 => println!("zero"),
    //     _ => {}
    // }
    // //等价于
    // if let 0 = i{
    //     println!("zero1")
    // }
    //panic表示不可恢复的错误
    //panic!("error occured");
    //可恢复的异常
    //let f = File::open("hello.txt");
    // match f {
    //     Ok(file)=>{
    //         println!("file name");
    //     }
    //     Err(err)=>{
    //         println!("err:{:?}",err);
    //     }
    // }
    //let f1 = File::open("hello.txt").unwrap();
    // fn f(i: i32) -> Result<i32, bool> {
    //     if i >= 0 {
    //         Ok(i)
    //     } else {
    //         Err(false)
    //     }
    // }
    // let res = f(-32);
    // match res {
    //     Ok(v) => println!("i={}", v),
    //     Err(erro) => println!("res is {}", erro)
    // }
    // fn open_file(path: &str) -> Result<String, io::Error> {
    //     let mut f = File::open(path)?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    //
    // let file = open_file("hello.txt");
    // match file {
    //     Ok(str0) => println!("str={}", str0),
    //     Err(erro) => {
    //         match erro.kind() {
    //             io::ErrorKind::NotFound => {
    //                 println!("not found file")
    //             }
    //             _ => println!("can't open file")
    //         }
    //     }
    // }
    // let r ;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r:{}",r);
    // let str;
    // {
    //     let str1 = "hello";
    //     let str2 = "lisisi";
    //     str = longer(str1,str2);
    //     println!("longer is {}",str);
    // }
    // let array  = [1,2,3,4,8,5,6];
    // let maxN = max(&array);
    // println!("maxN:{}",maxN)
    // let pa = Point { x: 1, y: 2 };
    // let pb = Point { x: 3, y: 4 };
    // println!("pa x = {0},y={1}", pa.get_x(), pa.y);
    // println!("pb x = {0},y={1}", pb.get_x(), pb.y);
    // let cali = Person { age: 32, name: String::from("raoshanshan") };
    // println!("cali:{}", output(cali))
    // let mut vector = vec![1,2,3,4];
    // // vector.push(5);
    // // vector.push(6);
    // // println!("vector:{:?}",vector)
    // println!("value:{}",match vector.get(9){
    //     Some(value)=>value.to_string(),
    //     None=>String::from("nothing")
    // })
    // let mut map = HashMap::new();
    // map.insert("haha",3);
    // map.insert("raoshanshan",5);
    // //println!("{:?}",map.get("lisisi").unwrap());
    // for p in map.iter(){
    //     println!("{:?}",p)
    // }
    // let newClass = ClassName::new(String::from("raoshanshan"));
    // println!("res:{}",newClass.public_method());
    // fn spaw_function(){
    //     for i in 0..5 {
    //         println!("spaw thread:{}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // }
    //
    // thread::spawn(spaw_function);
    // for i in 0..3{
    //     println!("main thread:{}",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    //闭包 | 之间是要传递的参数 |
    // let inc = | num:i32|->i32{
    //     num+1
    // };
    // println!("inc is {}",inc(5));
    // let i = 116i8;
    // let j = 0o106;
    // //println!("j={}",j);
    // let k = 2u16.pow(4);
    // println!("k={}",k);
    // let u = 0b101101u8.count_ones();
    // println!("u={}",u)
    // let i = 5f32.sqrt()*5f32.sqrt();
    // println!("i={}",i)
    // let i = false as i32;
    // assert_eq!(i,0)
    // let lazy:[i64;7] = [1,2,3,4,5,6,7];
    // assert_eq!(lazy[3],4)
    // let mut v = vec![1,2,3,4];
    // v.push(5);
    // v.push(6);
    // assert_eq!(v.iter().fold(1,|a,b|a*b),720);
    // let v1:Vec<i32> = (0..5).collect();
    // assert_eq!(v1,vec![0,1,2,3,4])
    let a = vec![0.0,0.707,-1.0,-0.707];
    let v = vec![0.0,0.707,1.0,0.707];
    let sa = &a;
    let sv = &v;

    fn print(n:&[f64]){
        for n1 in n {
            println!("{}",n1)
        }
    }
    print(sa);
    print(sv);
}

#[test]
fn test_longer(){
    assert_eq!(longer("abc","cd"),"abc");
}

//这段代码可能返回一个过期的引用
// fn longer(str1:&str,str2:&str)->&str{
//     if str2.len()>str1.len() {
//         str2
//     }else {
//         str1
//     }
// }

//用一个'加一个小写字母单词表示两个引用的生命周期一致
// 只要使用函数的地方和str1 str2的声明周期保持一致,那么还是可以继续使用的
fn longer<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str2.len() > str1.len() {
        str2
    } else {
        str1
    }
}

fn dangle() -> String {
    let str = String::from("haha");
    //str随着函数的结束,已经被释放了,所以它的引用也就是不被允许返回
    //&str
    str
}

//找出数组里面最大的值
trait Compare {
    fn compare(&self, object: &Self) -> i8;
}

impl Compare for u32 {
    fn compare(&self, object: &u32) -> i8 {
        if &self > &object {
            { 1 }
        } else if &self == &object {
            { 0 }
        } else {
            { -1 }
        }
    }
}

fn max<T:Compare>(array: &[T]) -> &T {
    let mut i = 1;
    let mut max_index = 0;
    while i < array.len() {
        if array[i].compare(&array[max_index]) == 1 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

fn another_function(x: i64, y: i64) {
    println!("x={0},y={1}", x, y);
}

/// example:
/// let x = add(1,2)
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

