pub trait CanCalcArea {
    fn calc_area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl CanCalcArea for Circle {
    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

pub struct Square {
    pub side_length: f64,
}

impl CanCalcArea for Square {
    fn calc_area(&self) -> f64 {
        self.side_length.powi(2)
    }
}

pub struct Rectangle {
    pub widht: f64,
    pub height: f64,
}

impl CanCalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.height * self.widht
    }
}
