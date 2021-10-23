#[derive(Debug)]
struct Cacher<T, U>
where
    T: Fn(u32) -> u32,
{
    calcular: T,
    valor: Option<U>,
}
impl<T, U> Cacher<T, U>
where
    T: Fn(u32) -> u32,
{
    fn nuevo(calcular: T, valor: U) -> Cacher<T, U> {
        Cacher {
            calcular,
            valor: None,
        }
    }
    fn valor(&mut self, arg: U) -> u32 {
        match self.valor {
            Some(v) => v,
            None => {
                let v = (self.calcular)(arg);
                self.valor = Some(arg);
                v
            }
        }
    }
}

fn generar(intensidad: u32) {
    let mut resultado = Cacher::nuevo(
        |num| {
            println!("Trabajando");
            num
        },
        None,
    );

    let res_a = resultado.valor(Some(4));
}

fn main() {
    generar(3);
}

// #[macro_use]
// extern crate rocket;

// //Uses
// use objects::{puntos, shapes};
// use rocket::serde::json::Json;
// //Mods
// mod objects;

// #[get("/world")]
// fn hi() -> Json<puntos::Point> {
//     let point2 = puntos::PointWithName {
//         point: puntos::Point { x: 99, y: 0 },
//         name: String::from("Un punto al azar"),
//     };
//     let point_cordinates = puntos::PointCordenates { cord1: 0, cord2: 0 };
//     let point1 = puntos::Point { x: -5, y: 2 };
//     let x_of_point_2 = return_point2(&point2.point);
//     let x_of_point_1 = return_point(&point1);
//     let x_of_point_cordinates = return_point(&point_cordinates);
//     println!(
//         "{}, {}, {}",
//         x_of_point_1, x_of_point_2, x_of_point_cordinates
//     );

//     let square1 = shapes::Square { side_length: 1f64 };
//     let circle1 = shapes::Circle { radius: 5f64 };
//     let rectangle1 = shapes::Rectangle {
//         height: 1f64,
//         widht: 2f64,
//     };
//     let (square_area, circle_area, rectangle_area) = (
//         get_area(&square1),
//         get_area(&circle1),
//         get_area(&rectangle1),
//     );
//     println!(
//         "El area del circulo es: {}, la del cuadrado:{} y la del rectangulo: {}",
//         circle_area, square_area, rectangle_area
//     );
//     Json(point1)
// }

// fn return_point(point: &impl puntos::IsPoint) -> &i32 {
//     point.x()
// }

// fn return_point2(point: &puntos::Point) -> &i32 {
//     &point.x
// }

// fn get_area(shape: &impl shapes::CanCalcArea) -> f64 {
//     shape.calc_area()
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hi])
// }
