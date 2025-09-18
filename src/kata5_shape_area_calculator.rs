// ============================================ 
// Kata 5: Shape Area Calculator
// Focus: Open/Closed Principle, Use of Traits
// ============================================ 

use std::f64::consts::PI;

/// Basic properties of a shape
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &str;
}

/// Circle
pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Result<Self, &'static str> {
        if radius <= 0.0 {
            Err("Radius must be positive")
        } else {
            Ok(Circle { radius })
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

/// Rectangle
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Result<Self, &'static str> {
        if width <= 0.0 || height <= 0.0 {
            Err("Width and height must be positive")
        } else {
            Ok(Rectangle { width, height })
        }
    }

    pub fn square(side: f64) -> Result<Self, &'static str> {
        Self::new(side, side)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

/// Triangle
pub struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    pub fn new(side_a: f64, side_b: f64, side_c: f64) -> Result<Self, &'static str> {
        if side_a <= 0.0 || side_b <= 0.0 || side_c <= 0.0 {
            return Err("All sides must be positive");
        }
        
        // Check triangle inequality
        if side_a + side_b <= side_c || 
           side_a + side_c <= side_b || 
           side_b + side_c <= side_a {
            return Err("Invalid triangle: sides do not satisfy triangle inequality");
        }
        
        Ok(Triangle { side_a, side_b, side_c })
    }

    pub fn equilateral(side: f64) -> Result<Self, &'static str> {
        Self::new(side, side, side)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Use Heron's formula
        let s = self.perimeter() / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

/// Statistics of a collection of shapes
pub struct ShapeCollection {
    shapes: Vec<Box<dyn Shape>>,
}

impl ShapeCollection {
    pub fn new() -> Self {
        ShapeCollection { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()
    }

    pub fn total_perimeter(&self) -> f64 {
        self.shapes.iter().map(|s| s.perimeter()).sum()
    }

    pub fn summary(&self) -> String {
        let summaries: Vec<String> = self.shapes
            .iter()
            .map(|s| format!("{}: area={:.2}, perimeter={:.2}", 
                           s.name(), s.area(), s.perimeter()))
            .collect();
        summaries.join("\n")
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;

    #[test]
    fn test_circle_calculations() {
        let circle = Circle::new(1.0).unwrap();
        assert!((circle.area() - PI).abs() < 0.001);
        assert!((circle.perimeter() - 2.0 * PI).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_calculations() {
        let rect = Rectangle::new(3.0, 4.0).unwrap();
        assert_eq!(rect.area(), 12.0);
        assert_eq!(rect.perimeter(), 14.0);
    }

    #[test]
    fn test_shape_collection() {
        let mut collection = ShapeCollection::new();
        collection.add(Box::new(Circle::new(1.0).unwrap()));
        collection.add(Box::new(Rectangle::square(2.0).unwrap()));
        
        let total_area = collection.total_area();
        assert!(total_area > 7.0 && total_area < 8.0); // π + 4
    }
}
