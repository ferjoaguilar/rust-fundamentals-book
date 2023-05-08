struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 { // &self is a reference to the struct
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }

    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x: x, y: y, radius: radius }
    }
}

fn impl_methods() {
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    let g = c.grow(2.00).area();
    println!("{}", g);

    let n = Circle::new(0.5, 0.0, 25.2);
    println!("{}", n.area());
}