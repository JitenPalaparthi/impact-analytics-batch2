pub mod rect;
pub mod shapes;
pub mod square;
use rect::Rect;

fn main() {
    let r1 = Rect::new(12.4, 14.5);
    print_shape1(&r1);
    print_shape2(&r1);
    print_shape3(&r1);
    print_shape4(&r1);

    let s1 = crate::square::Square::new(12.54);

    print_shape1(&s1);
    print_shape2(&s1);
    print_shape3(&s1);
    print_shape4(&s1);
}

fn print_shape1<T: shapes::Shapes>(t: &T) {
    // compile time bounds
    println!("Area:{:.3} Perimeter:{:.3}", t.area(), t.perimeter())
}

fn print_shape2<T>(t: &T)
where
    T: shapes::Shapes,
{
    // compile time bounds
    println!("Area:{:.3} Perimeter:{:.3}", t.area(), t.perimeter())
}

fn print_shape3(t: &impl shapes::Shapes) { // Compiler generates the code
    // compile time bounds
    println!("Area:{:.3} Perimeter:{:.3}", t.area(), t.perimeter())
}

fn print_shape4(t: &dyn shapes::Shapes) { // It used VTables so that at runtime, it understand which method to be called
    // compile time bounds
    println!("Area:{:.3} Perimeter:{:.3}", t.area(), t.perimeter())
}


