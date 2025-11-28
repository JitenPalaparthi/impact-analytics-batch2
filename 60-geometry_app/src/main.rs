use jp_shapes::{Circle, Rectangle, Shape, shape,Greet};

fn main() {
    Greet();
    // Using constructors
    let c1 = Circle::new(1.0);
    let r1 = Rectangle::new(3.0, 4.0);

    // Using the macro
    let c2 = shape!(circle: 2.5);
    let r2 = shape!(rect: 5.0, 6.0);

    let shapes: Vec<(&'static str, &dyn Shape)> = vec![
        ("c1", &c1),
        ("r1", &r1),
        ("c2", &c2),
        ("r2", &r2),
    ];

    for (name, s) in shapes {
        println!(
            "{} -> area = {:.3}, perimeter = {:.3}",
            name,
            s.area(),
            s.perimeter()
        );
    }
}