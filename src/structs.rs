struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Color(i32, i32, i32);

fn structures(){
    // Inmutable struct
    let origin = Point { x: 0, y: 0 };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    // Mutable struct
    let mut origin = Point { x: 0, y: 0 };
    origin.x = 5;
    origin.y = 258;
    println!("The point is at ({}, {})", origin.x, origin.y);

    let mut point = Point3D { x: 0, y: 0, z: 0 };
    point = Point3D { y: 1, .. point };

    println!("The point is at ({}, {}, {})", point.x, point.y, point.z);

    let black = Color(0, 0, 0);
    println!("The color is ({}, {}, {})", black.0, black.1, black.2);

}