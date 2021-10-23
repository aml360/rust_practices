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
