fn main() {
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

    let a = [10, 20, 30, 40, 50];
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
    let str = "broadcast";
    //表示从0到第5个元素,但不包括第5个元素
    let str1 = &str[0..5];
    let str2 = &str[5..9];
    println!("str1={0},str2={1}",str1,str2);
}

fn dangle() -> String {
    let str = String::from("haha");
    //str随着函数的结束,已经被释放了,所以它的引用也就是不被允许返回
    //&str
    str
}

fn another_function(x: i64, y: i64) {
    println!("x={0},y={1}", x, y);
}

/// example:
/// let x = add(1,2)
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

