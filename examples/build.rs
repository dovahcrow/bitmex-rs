extern crate bitmex;
extern crate dotenv;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::ErrorKind;
use std::env::var;

use bitmex::core::utils;
use bitmex::{BitMEX, Result};


//-----------Libra start------------
#[macro_use]
extern crate assert_matches;
use assert_approx_eq::assert_approx_eq;
use serde::de::Unexpected::Str;


//-----------Libra   end------------

//静态变量的名称需要大写，不然就会有警告提示。
//需要指定变量类型,编译器不推导
//无法通过mut使常量可变
//内联方式
const MAX_LINES : u32 = 100_000;

fn main(){
    bitmex::print_version();
    utils::print_title("Hello Rust");
    let x = String::from("");

    fn is_hello<T: Into<Vec<u8>>>(s: T) {
        let bytes = b"hello".to_vec();
        assert_eq!(bytes, s.into());
    }

    //test_base();
    //test_ref();
    //test_tuple();
    //test_ary();
    //test_slice();
    //test_str();
    //test_struct();
    //test_enum();
    //test_match();
    //test_common();
    //test_panic();
    //test_trait();

    //test_libra();
}





fn test_base(){
    /*
        rust基本类型：这些类型的变量都存储在栈上（这句话脑子记一下，以后很有用）
        整数类型：u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, [isize,usize--这两个变量占用空间跟机器字长有关] (u表示无符号的） 默认是i32
        浮点类型：f32, f64   默认是f64
        布尔类型：bool（false，true）
        字符类型：char
    */
    {
        let n = 5;    //编译器会自动推导n的默认类型
        let n: i32 = 5;   //显示指明变量类型，可以不指明，一般不用显示指明，看下面就知道了
        let n: i64 = 500;
        let n: usize = 700;
        n+1;

        let f = 2.3;
        let b = false;
        let c = 'a';
        let cc : u8 = b'c'; //此处必须这么写，否则类型不匹配

        //后缀形式(整数类型和浮点数类型都可以使用后缀形式表达，其他的不可以哦)
        let n = 23i32;
        let f = 0.23f32;
        let c = b'a';   //只有这一种u8类型的变量没有办法使用后缀形式表达

        //一次为多个变量进行赋值，猜一猜他们各自是什么类型
        let (n, f, b, c, cc) = (34u32, 2.4, true, b'k', '😻');
        println!("{},{},{},{},{}", n, f, b, c, cc);
    }

    /*-------------------------------------------------------------------------------------------------------*/
    /*----------以上只要关心基本类型和后缀表达，其他不要纠结，后面会讲解，整理思绪，take it easy ！----------*/
    /*-------------------------------------------------------------------------------------------------------*/

    /*
        重要知识点：
        不可变变量和可变变量 默认不可变
    */
    {
        let x = 5;  //x为默认不可变变量
        //x = 6;    //Error,因为x是不可变的，也就是说x无论在哪儿都是只读的，只能使用无法修改。除非重新定义x  （rust语言的安全性开始体现了）
        //以下是重新定义x
        let x = 3;
        let x = false;
        //x = true; //Error,同理
        let x = 3.4;
        let x = 'c';
        println!("{}", x);

        let mut x = 5;  //mut关键字，x重新定义为可变变量，可变变量可以修改值，但类型必须一致，除非重新定义x
        x = 23u8;
        //x = 34u64;    //Error,类型不一致
        //x = 'c';      //Error,类型不一致
        //以下是重新定义x
        let mut x = 'c';
        x = 'u';
        //x = b'u';     //Error,类型不一致
        let mut x = false;
        x = true;

        //此处注意
        let x = 23;
        //x = 45;   //Error,思考一下是什么原因呢？

        fn test_immutable(i : i32){
            //i = 32;   //Error,思考一下是什么原因呢？
        }
        fn test_mutable(mut i : i32){
            i = 32;
        }
        let x = 32;
        test_mutable(x);
        let mut x = 32;
        test_mutable(x);    //如果从栈变量的值Copy来看，由于发生了Copy，原先变量x是否可变不重要了，拷贝后的变量i是否可变是关键
    }

    /*
        重要知识点：
        Copy（拷贝）和Move（所有权转移）
    */
    {
        //以下就是Copy，第一这些变量是在栈上的，拷贝非常的快，不涉及所有权的转移，第二基础类型的变量都是在栈上的
        let x = 5;
        let y = x;

        let x = true;
        let y = x;
        //y = false;    //Error,因为y是不可变的

        fn test_stack(i : bool) {
            //i = false;    //Error,思考一下为什么哦
            println!("{}",i)
        }
        test_stack(y);    //此时也是参数变量的值拷贝，（各自还是各自，互不干扰，只不过是值相同而已）
        println!("{},{}",x,y);


        //引入第一在堆上的变量
        let s = String::from("heap");
        let s1 = s;     //此处就不是Copy了，此处是Move，所有权发生了转移，s被丢弃将无法使用，除非重新定义s
        //println!("{}",s);     //Error，s已经被丢弃了，丧失了对"heap"的所有权

        //重新定义了s，此时s与s1互不相关了
        let s = String::from("new heap");
        println!("{}",s);

        fn test_heap(str : String){
            println!("{}",str);
        }
        test_heap(s);  //此处也不是参数变量的值拷贝了，是Move，是所有权发生了转移，转移到了test_heap函数的变量中，s被丢弃了将无法使用，除非重新定义s
        //println!("{}",s);     //Error，s已经被丢弃了，丧失了对"new heap"的所有权

        /*
            总结：
            因为栈上的变量Copy速度很快，所以就各自拥有各自的所有权，不会有所有权的转移
            堆上的变量的操作速度相对慢很多，体量又有可能很大，因此不拷贝，所以就有了所有权的转移。所有权的转移也是体现了rust语言的安全性
        */
    }
}

//元组
fn test_tuple(){
    //元组在声明时就必须确定各个值，且一旦声明结束，空间大小就是固定的，后续不能再增大或减小
    {
        let x: (i32, char, bool) = (23, 'a', true);
        let x = (12i8, 7.88, String::from("tuple"));
        let (a, b, c) = x;  //此处有变量是Copy（a,b）,有变量是Move(c),所以x只是被部分Move了，[x.]还可以使用
        println!("{},{},{},{},{}", a, b, c, x.0, x.1);
        //println!("{}",x.2);   //Error,变量x.2已经Move了
        //println!("{:?}",x);   //Error,变量x由于已经被部分Move了，所以也无法再单独使用了
    }

    {
        let x = (12i8, 7.88, true);
        let y = x;   //由于此元组元素是基本类型存储在栈上，所以是全量Copy，没有所有权的转移, 相当于新建了个副本互不干扰

        let x = (12i8, 7.88, String::from("new tuple"));
        let y = x;  //此时由于有堆上数据，所以完全被Move了，x被丢弃无法被使用
    }

    {
        let x = (45, true, 'c');
        fn test_copy(tup: (i32, bool, char)) {
            //只读，无法修改
        }
        test_copy(x);
        print!("{:?}", x);

        let x = (45, true, String::from("mut tuple"));
        fn test_move(tup: (i32, bool, String)) {
            //只读，无法修改
        }
        test_move(x);
    }

    {
        //注意：元组也满足不可变和可变性
        let mut x = (32, false, String::from("mut tuple"));
        x.0 = 34;
        x.2.push_str(" push");
        x = (45, true, String::from("rust"));
    }
}

//引用
fn test_ref(){
    {
        //引用不涉及到所有权的Move
        let x = 32;
        let y = &x;
        let z = &y;
        println!("{:p},{:p},{:p}", &x, y, z);
        println!("{},{},{}", x, y, z);

        let x = String::from("ref");
        let y = &x;
        let z = &y;
        println!("{:p},{:p},{:p}", &x, y, z);
        println!("{},{},{}", x, y, z);
    }

    {
        //基本栈类型也是同样的
        let mut x= String::from("ref");
        let x1 = &x;
        {
            //由于有下面可变引用y的存在
            //所以，只有此范围可以对x1进行操作
            //所以，此范围可以对x进行操作，出了此范围x只能被引用
        }
        let y= &mut x;
        //let mut y = &mut x;   //没有必要这么写
        //let z= &mut x;  //同时定义两个可变引用不会有错，但是用的时候就会编译不通过(因为可能会同时改变同一块内存)，所以不要同时声明两个同样的可变引用
        test_mutable_ref(y);

        fn test_mutable_ref(str : &mut String){
            str.push_str("go");
        }
        println!("{}",y);

        //println!("{}",x1);    //Error,由于可变引用y的存在
        let z = &x;     //y的使用范围到此处截止，下面将无法使用
        //println!("{}",y);     //Error,由于不可变引用z的存在

        /*
            总结：
                1.可变引用的写法
                2. 两大限制：不要同时有两个可变引用；存在一个可变和一个不可变时，使用范围有限制
        */
    }
}

//切片
fn test_slice(){
    //切片是一种特殊的引用,允许引用集合类型的部分元素,由于是引用所以没有（所有权）的概念,但是还是有（可不可变）的概念的
    //String Slice
    {
        let s = String::from("hello rust");
        let s0 = "hello rust";      //字符串字面常量其实也是个引用
        let s1 = &s[0..5];
        let s2 = &s[..3];
        let s3 = &s[..=3];
        let s4 = &s[3..s.len()];
        let s5 = &s[3..];
        //let s5 = s0[3..];     //Error,写法错误
        println!("{},{},{},{},{},{},{}", s, s0, s1, s2, s3, s4, s5);

        let s6 = String::from("hello rust");
        //let s7 = s0 + s6;          //Error,字符串加法必须以String类型开头，后面相加的必须是&str
        let s7 = s + &s6 + s0 + " end";   //此处编译器会将&String类型强制转换成&str，同时s的所有权Move了
        //println!("{}",s);         //Error,s被丢弃了
        println!("{}", s7);
    }

    //Array Slice
    {
        let ary = [12 ,13, 14, 15, 16, 17];
        let ary1 = &ary[2..5];
    }

}

//字符串
fn test_str(){
    let s = String::from("Libra");
    s.len();
    s.capacity();
    let mut s_r = "Libra";
    s_r.len();
    s_r = &s;   //&String可以被强制转换成&str，反则不可以
    let tt : &str;





    let s0 = "Libra";      //字符串字面常量其实也是个引用
    let s1 = s.clone();     //深拷贝

    let f1 = 2.3;
    let i1 = 5;
    let com_str = format!("{}--{}--{}",f1,i1,"zz");     //这种组合构造方式不错哦

}

//数组
fn test_ary(){
    /*
    使用array的场景：
        1.元素个数确定时，例如一年的12个月的名称
        2.希望数据分配在栈上而不是堆上时(以下有数组元素在堆上的例子)
    */
    {
        //数组类型必须一致,且它们的大小在编译时会被确定
        //一维数组
        let ary = [12, 13, 14];
        let ary: [f64; 5] = [23.4, 45.6, 78.5, 34.5, 12.3];
        let ary: [f64; 5] = [2.3; 5];     //对5个数初始化2.3
        //ary[0] = 9.1;     //Error,不可变数组
        //let a = ary[10];    //Error,超出索引范围
        let ary1 = ary; //Copy
        println!("{:?}", ary);

        //二维数组
        let ary:[[i32;3];2] = [[5;3];2];
        let ary:[[i32;3];2] = [[5,5,5],[5,5,5]];  //和上面一样
    }

    {
        let ary = [String::from("Btc"), String::from("Eos"), String::from("Libra")];
        //let ary = [String::from("Etc"); 5];   //Error,元素没有Copy的特性
        let ary1 = ary; //Move
        //println!("{:?}", ary);    //Error,所有权转移了
    }

    {
        let mut ary = [5;3];    //只可以修改值，不能删除和增加
        ary[0] = 10;
    }

    {
        let ary = [String::from("Btc"), String::from("Eos"), String::from("Libra"), String::from("Etc")];
        fn test_ref(ary: &[String]) {
            return;
        }
        fn test_ref_len(ary: &[String;2]) {
            return;
        }
        fn test(ary: [String;4]){
            return;
        }
        test_ref(&ary);
        test_ref(&ary[2..4]);
        test(ary);  //Move
        //test_ref_len(&ary);        //Error,无法匹配
        //test_ref_len(&ary[1..3]);  //Error,无法匹配
        let ary = [String::from("Btc"), String::from("Eos")];
        test_ref_len(&ary);
    }
}


//结构体
fn test_struct(){
    #[derive(Debug)]
    struct User{
        name : String,
        age: i32,
    }

    struct Car {
        name : &'static str,
        age : i32,
    }


    // 匿名成员结构体,又叫“元组结构体”
    struct Point2(i32, i32);
    let p = Point2(23, 45);
    let p = Point2::new(12, 67);
    println!("{}", p.1);

    impl User{
        //Self指的是类
        pub fn new(name : String , age : i32) -> Self{
            User{name,age}   //同名可以省略key，否则形式上必须是key:value
        }

        //&self指的是类对象
        fn change_user_name(&mut self,new_name:String){
            self.name = new_name;
        }

    }

    impl Point2{
        pub fn new(x : i32, y : i32) -> Self{
            Point2(x, y)    //此处不能有分号
        }
    }

    let mut user = User::new(String::from("shilf"),32);
    let mut user = User{ name:String::from("shilf"), age:41};
    let user1 = User{ name:String::from("shily"), ..user};      //此处只是做了一次复制，没有任何关联关系
    println!("{:#?}",user1);
    user.change_user_name(String::from("shilp"));
    user.name.push_str(" &");
    println!("{:#?}",user);
    println!("{:#?}",user1);
}

//枚举
fn test_enum(){
    enum MyLanguage{
        Java(i32),
        Rust(i32),
        Python,
        C(i32,String),                  //元组
        Julia{version:i32,name:String}, //匿名元组结构体
    }
    let l_rust = MyLanguage::Rust(2);

    //注意此处不能倒置
    if let MyLanguage::Python = l_rust {
        println!("Python!");
    }else {
        println!("Other Language!");
    }

    fn language(language : MyLanguage){
        match language{
            MyLanguage::Rust(version) => println!("{}",version),
            other_language_version => (),
        }
    }

}

fn test_common(){
    //statement(语句)和expression(表达式)的关系，statement执行一些动作但不会得到一个可返回的值，expression会得到一个可返回的结果值
    //statement是以分号结束的，但expression没有结尾的分号

    //rust没有(a > 0 ? true : false)这种条件表达式
    //rust的条件表达式，条件必须是bool类型
    let n = if 20 > 0 {
        1
    }else {
        2
    };
}

fn test_match(){
    match File::open("xx.txt"){
        Ok(_) => println!("open success"),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(_) => println!("create success"),
                Err(e) => println!("create fail {:#?}", e),
            },
            other_err => println!("open fail {:#?}", other_err),
        },
    }
}

fn test_panic(){
    panic!("crash and exit");
}

//宏
fn test_macro(){
    macro_rules! ttg {
        () => {};
    }
}

fn test_trait(){
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }
    let x = String::from("pp");
    &x as &Foo;


}


//---------------------------------Libra----------------------------
fn test_libra(){
    test_assert_approx_eq(2.000000001f64,2.000000002f64, None);
    //test_assert_approx_eq(2.000000001f64,2.000000002f64, Some(1.0e-16));
}

fn test_assert_approx_eq(x : f64, y : f64, diff : Option<f64>){
    //判断两个数大致相等，若不大致相等，会panic
    //默认diff是1.0e-6
    match diff {
        None => assert_approx_eq!(x,y),
        _ => assert_approx_eq!(x,y,diff.unwrap()),
    };
}

fn test_assert_match(){

}

