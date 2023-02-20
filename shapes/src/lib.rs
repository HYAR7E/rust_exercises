const PI: f32 = std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Square {
    size: f32,
}

#[derive(Clone, Copy)]
pub struct RightAngleTriangle {
    up: f32,
    along: f32,
}

#[derive(Clone, Copy)]
pub struct Circle {
    radius: f32,
}

impl Square {
    pub fn new(size: f32) -> Self {
        Square { size }
    }

    pub fn area(&self) -> f32 {
        self.size.powi(2)
    }

    pub fn perimeter(&self) -> f32 {
        self.size * 4.0
    }
}

impl RightAngleTriangle {
    pub fn new(up: f32, along: f32) -> Self {
        RightAngleTriangle { up, along }
    }

    pub fn area(&self) -> f32 {
        self.up * self.along / 2.0
    }

    pub fn perimeter(&self) -> f32 {
        let hypotenuse = (self.up.powi(2) + self.along.powi(2)).sqrt();
        self.up + self.along + hypotenuse
    }
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }

    pub fn area(&self) -> f32 {
        self.radius.powi(2) * PI
    }

    pub fn perimeter(&self) -> f32 {
        self.radius * 2.0 * PI
    }
}

pub enum Shape {
    Square(Square),
    SquareTriangle(RightAngleTriangle),
    Circle(Circle),
}

impl Shape {
    pub fn area(&self) -> f32 {
        match self {
            Shape::Square(square) => square.size.powi(2),
            Shape::SquareTriangle(rat) => rat.up * rat.along / 2.0,
            Shape::Circle(circle) => PI * circle.radius.powi(2),
        }
    }

    pub fn perimeter(&self) -> f32 {
        match self {
            Shape::Square(square) => square.size.powi(2),
            Shape::SquareTriangle(rat) => {
                let hypotenuse = (rat.up.powi(2) + rat.along.powi(2)).sqrt();
                rat.up + rat.along + hypotenuse
            }
            Shape::Circle(circle) => 2.0 * PI * circle.radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let s1 = Square::new(4.0);
        assert_eq!(s1.area(), 16.0);
        assert_eq!(s1.perimeter(), 16.0);
    }

    #[test]
    fn right_angle_triangle() {
        let rat = RightAngleTriangle::new(4.0, 3.0);
        assert_eq!(rat.area(), 6.0);
        assert_eq!(rat.perimeter(), 12.0);
    }

    #[test]
    fn circle() {
        let c1 = Circle::new(4.0);
        assert_eq!(c1.area(), 50.265484);
        assert_eq!(c1.perimeter(), 25.132742);
    }

    #[test]
    fn shape_square() {
        let square = Square::new(4.0);
        let shape_square = Shape::Square(square);
        assert_eq!(square.area(), shape_square.area());
        assert_eq!(square.perimeter(), shape_square.perimeter());
    }

    #[test]
    fn shape_right_angle_triangle() {
        let rat = RightAngleTriangle::new(4.0, 4.0);
        let shape_rat = Shape::SquareTriangle(rat);
        assert_eq!(rat.area(), shape_rat.area());
        assert_eq!(rat.perimeter(), shape_rat.perimeter());
    }

    #[test]
    fn shape_circle() {
        let circle = Circle::new(4.0);
        let shape_circle = Shape::Circle(circle);
        assert_eq!(circle.area(), shape_circle.area());
        assert_eq!(circle.perimeter(), shape_circle.perimeter());
    }
}
