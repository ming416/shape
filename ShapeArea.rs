//定义trait 计算图形面积
trait ShapeArea {
    fn area(&self) -> f32;
}

struct Rectangle { pub width: f32, pub height: f32 }
struct Triangle { pub side: f32 }
struct Circle { pub radius: f32 }

impl ShapeArea for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl ShapeArea for Triangle {
    fn area(&self) -> f32 {
        self.side * 0.5 * 3.0_f32.sqrt() / 2.0 * self.side
    }
}

impl ShapeArea for Circle {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

// 泛型必须实现 HasArea
fn area<T: ShapeArea>(t: &T) -> f32 {
    // 调用泛型约束的方法
    t.area()
}

fn main() {

   let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    
   println!("area of rectangle is  :{}",area(&rectangle));
}
