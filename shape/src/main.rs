#[derive(PartialEq, Eq)]
enum ShapeEnum {
    RECTANGLE,
    CIRCLE
}

struct Shape {
    kind: ShapeEnum,
    dimensions: (f32, f32)
}

impl Shape {
    fn area(&self) -> f32 {
        if self.kind == ShapeEnum::RECTANGLE {
            let (width, height) = self.dimensions;
            return width * height;
        } else if self.kind == ShapeEnum::CIRCLE {
            let (pi, radious) = self.dimensions;
            return pi * (radious * radious);
        }
        
        0.0
    }
}

const PI: f32 = 3.14;

fn main() {
    let rectangle: Shape = Shape { kind: ShapeEnum::RECTANGLE, dimensions: (10.0, 20.0) };
    println!("The area of the rectangle is {}", rectangle.area());
    
    let circle: Shape = Shape { kind: ShapeEnum::CIRCLE, dimensions: (PI, 10.0) };
    println!("The area of the cirlce is {}", circle.area());
}
