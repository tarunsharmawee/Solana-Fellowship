enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape 
    
    let ans = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,  
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side, 
    };
    
    return ans;
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let area = calculate_area(circle);
    println!("Area of the circle: {}", area);

    let area = calculate_area(square);
    println!("Area of the square: {}", area);

    let area = calculate_area(rectangle);
    println!("Area of the rectangle: {}", area);
}