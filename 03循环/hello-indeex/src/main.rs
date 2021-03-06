fn main() {
    //作用域块
    let a = {
        println!("ABC"); // 输出 ABC
        "DEF" // 作为这个花括号块的值
    }; // 分号不可省略
    println!("{}", a);

    //if else
    let a1 = 1;
    let b1 = 2;
    let c1 = if a1 < b1 {
        println!("true");
        "ABC"
    } else {
        println!("false");
        "DEF"
    };
    println!("{}", c1);
    let a2 = 1;
    let b2 = 2;
    if a2 < b2 {
        println!("if true 1");
    } else if a2 == b2 {
        println!("if true 2");
    }

    //match(类似switch)
    let week = "mon";
    let a3 = match week {
        "sun" => "weekend",
        "mon" | "tue" | "wed" | "thu" | "fri" => "weekday",
        "sat" => "weekend",
        _ => "illegal",
    }; //在表达式值为字符串或者数值时，末尾需要有默认分支 _ 。
    println!("{}", a3);
    //默认分支也可以用一个变量代替，这样，变量会被赋予表达式值
    let day = "xxx";
    match day {
        "sun" | "sat" => {
            println!("weekend");
        }
        "mon" | "tue" | "wed" | "thu" | "fri" => {
            println!("weekday");
        }
        other => {
            println!("{} is illegal", other);
        }
    }; //如果表达式的值为 bool 类型，枚举完 true 和 false 两个分支，不需要默认分支

    //while
    let mut num1 = 0;
    while num1 < 10 {
        println!("{}", num1);
        num1 += 1;
    }

    //loop
    let mut num2 = 0;
    loop {
        println!("{}", num2);
        num2 += 1;
        if num2 < 10 {
            continue;
        }
        break;
    }
    //loop表达式
    let mut num3 = 0;
    let num4 = loop {
        println!("{}", num3);
        num3 += 1;
        if num3 == 10 {
            break num3;
        }
    };
    println!("{}", num4);

    //for in循环
    let vec5 = vec![1, 2, 3];
    for v in &vec5 {
        println!("{}", v);
    }
    for v1 in 10..20 {
        println!("{}", v1);
    }
}
