struct Size {
    width: f32,
    height: f32,
}

// struct Size(f32, f32);//匿名

struct Product {
    name: String,
    price: u32,
}

// enum Shape {
//     Rectangle {
//         // Rectangle 附有 width 和 height 两个字段
//         width: f32,
//         height: f32,
//     },
//     Square(f32), // Square 只附有一个字段
//     Round(f32),  // Round 只附有一个字段
//     Point,       // Point 没附字段
// }

fn main() {
    //struct
    let size1 = Size {
        width: 2.0,
        height: 3.4,
    };
    let area = size1.width * size1.height;
    println!("{}", area);

    let my_product = Product {
        name: format!("Rust Book"),
        price: 30,
    };
    let name: &str = &my_product.name;
    let price = my_product.price;
    println!("{}: {}", name, price);

    //enmu
    // let my_shape1 = Shape::Rectangle {
    //     width: 30.0,
    //     height: 20.0,
    // };
    // let area1 = match my_shape1 {
    //     Shape::Rectangle { width, height } => width * height,
    //     Shape::Square(len) => len * len,
    //     Shape::Round(r) => r * r * std::f32::consts::PI,
    //     Shape::Point => 0.,
    // };
    // println!("{}", area1);
}
