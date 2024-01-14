use enum_impl::EnumImpl;

#[derive(EnumImpl)]
pub enum Shape {
    #[enum_impl(pub is)]
    Circle(f64),
    #[enum_impl(pub as_ref, pub as_ref_mut, impl from)]
    Rectangle(f64, f64),
    #[enum_impl(pub as_ref, pub as_ref_mut, pub into)]
    Square(f64),
    #[enum_impl(pub from = "create_cuboid", pub is, pub into)]
    Cuboid { width: f64, height: f64, depth: f64 },
}

fn main() {
    let circle = Shape::Circle(5.0);
    assert!(circle.is_circle());

    let rectangle = Shape::from((3.0, 4.0));
    assert_eq!(rectangle.as_rectangle().unwrap(), (&3.0, &4.0));

    let mut square = Shape::Square(10.0);
    assert_eq!(*square.as_square().unwrap(), 10.0);

    *square.as_square_mut().unwrap() = 15.0;
    assert_eq!(*square.as_square().unwrap(), 15.0);
    assert_eq!(square.into_square().unwrap(), 15.0);

    let cuboid = Shape::create_cuboid(2.0, 3.0, 4.0);
    assert!(cuboid.is_cuboid());

    let (width, height, depth) = cuboid.into_cuboid().unwrap();
    assert_eq!(width, 2.0);
    assert_eq!(height, 3.0);
    assert_eq!(depth, 4.0);
}
