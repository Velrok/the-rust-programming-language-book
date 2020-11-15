use std::fmt;

#[derive(Debug)]
enum Temp {
    C(f64),
    F(f64),
}

impl fmt::Display for Temp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Temp::C(c) => write!(formatter, "{}°C", c),
            Temp::F(f) => write!(formatter, "{}°F", f),
        }
    }
}


fn main() {
    let london = Temp::C(8.0);
    let orlando = Temp::F(80.0);

    println!("Temperature in London {}({})", as_c(london), london);
    println!("Temperature in Orlando {}({})", as_c(orlando), orlando);
}

fn as_c(t: &Temp) -> Temp {
    let tt = t.copy();
    match tt {
        Temp::C(x) => Temp::C(x),
        Temp::F(x) => Temp::C(x),
    }
}

fn to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
