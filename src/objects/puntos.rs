use std::ops::Deref;

use rocket::serde::Serialize;

pub trait IsPoint {
    fn x(&self) -> &i32;
    fn y(&self) -> &i32;
}

#[derive(Serialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl IsPoint for Point {
    fn x(&self) -> &i32 {
        &self.x
    }
    fn y(&self) -> &i32 {
        &self.y
    }
}

#[derive(Debug)]
pub struct PointWithName {
    // use Point::*
    pub point: Point,
    pub name: String,
}

impl Deref for PointWithName {
    type Target = Point;
    fn deref(&self) -> &Point {
        &self.point
    }
}

fn test() {
    let p = PointWithName {
        point: Point { x: 0, y: 1 },
        name: String::from("Oeoe"),
    };
    // let x = p.x;
    
}

// impl IsPoint for PointWithName {
//     fn x(&self) -> &i32 {
//         &self.point.x
//     }
//     fn y(&self) -> &i32 {
//         &self.point.y
//     }
// }
pub struct PointCordenates {
    pub cord1: i32,
    pub cord2: i32,
}

impl IsPoint for PointCordenates {
    fn x(&self) -> &i32 {
        &self.cord1
    }
    fn y(&self) -> &i32 {
        &self.cord2
    }
}

pub struct Square {}
