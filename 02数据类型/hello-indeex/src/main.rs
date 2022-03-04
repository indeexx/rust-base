fn main() {
    //数值类型
    let num1: u8 = 1;
    let num2: f32 = 10.1;
    //字符串
    let str = "indeex";
    //布尔
    let b: bool = true;
    println!("{}", num1);
    println!("{}", num2);
    println!("{}", str);
    println!("{}", b);
    let b: bool = false;
    println!("{}", b);

    //类型转换
    let c = num1 as f32 * num2 * 2.0;
    println!("{}", c);

    //元组
    let tuple1: (f32, u32) = (10.5, 2);
    let tuple2: (f32, _) = (12.5, 3); //也可以只写一部分，省略的部分可以用 _ 表示
    println!("{}", tuple1.1);
    let (tp11, tp12) = tuple1; //解构
    let (tp21, tp22) = tuple2;
    println!("{}", tp11);
    println!("{}", tp12);
    println!("{}", tp21);
    println!("{}", tp22);

    //数组
    //定长数组
    let arr: [u32; 4] = [1, 2, 3, 4];
    let arr1: [u32; 4] = [1; 4];
    println!("{}", arr[3]);
    // 下标可以是变量，但是必须是 usize 类型的
    let index: usize = 0;
    println!("{}", arr1[index]);
    //不定长数组，可以使用push pop等方法
    let vec: Vec<u32> = vec![100, 200, 300, 400];
    println!("{}", vec[index]);
    println!("{}", vec.len());

    //字符串
    let str1: String = String::from("这是字符串");
    println!("{}", str1);
    let a = "hello";
    let b = "indeex";
    let str2 = format!("{}空格{}", a, b); //format!类似printIn!
    println!("{}", str2);
    let word = format!("X");
    println!("{}", word.len()); //英字占字节数
    let hanzi = format!("字");
    println!("{}", hanzi.len()); //汉字占字节数
    let str3 = format!("这样搜索字符");
    println!("{}", str3.chars().nth(3).unwrap()); //开销大

    //克隆字符串
    let str_clone = format!("ABC");
    let str1_clone = str_clone.clone();
    println!("{}", str1_clone);
    let str_clone = format!("XYZ");
    println!("{}", str_clone);

    //引用
    //不可变引用
    let str4 = format!("EFG");
    {
        let str5: &String = &str4; // 不可变引用str5
        let str6 = &str4; // 另一个不可变引用str6
        println!("{} {}", str5, str6);
    }
    //可变引用
    let mut str7 = format!("ABC");
    {
        let str8: &mut String = &mut str7;
        *str8 = format!("DEF");
    }
    println!("{}", str7);

    //去除空格
    let white_str = format!(" HIJ ");
    let w_str: &str = white_str.trim();
    println!("{}", w_str);

    //带转义符字符串
    let str10: &str = r#"
这是一行。
这又是一行。
"#;
    println!("{}", str10);
}
