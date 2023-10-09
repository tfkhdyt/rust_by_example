use std::fmt::{self, Display};

// #[derive(Debug)]
// struct MinMax(i64, i64);

// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.0, self.1)
//     }
// }

// #[derive(Debug)]
// struct Point2D {
//     x: f64,
//     y: f64,
// }
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "x: {}, y: {}", self.x, self.y)
//     }
// }

// #[derive(Debug)]
// struct Complex {
//     real: f64,
//     imag: f64,
// }
// impl fmt::Display for Complex {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} + {}", self.real, self.imag)
//     }
// }

// struct Structure(i32);
// impl fmt::Display for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let vec = &self.0;
//         write!(f, "[")?;
//         for (count, v) in vec.iter().enumerate() {
//             if count != 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}: {}", count, v)?;
//         }
//         write!(f, "]")
//     }
// }

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{:0>2X}{:0>2X}{:0>2X}",
            self.red, self.green, self.blue
        )
    }
}

fn main() {
    // for city in [
    //     City {
    //         name: "Dublin",
    //         lat: 53.347776,
    //         lon: -6.259722,
    //     },
    //     City {
    //         name: "Oslo",
    //         lat: 59.95,
    //         lon: 10.75,
    //     },
    //     City {
    //         name: "Vancouver",
    //         lat: 49.25,
    //         lon: -123.1,
    //     },
    // ] {
    //     println!("{}", city);
    // }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // println!("{:?}", color);
        println!("{}", color);
    }

    // let v = List(vec![1, 2, 3]);
    // println!("{}", v);

    // println!("Hello, world!");
    // println!("I'm a Rustacean!");

    // comments
    // let x = 5 + /* 90 + */ 5;
    // println!("Is `x` 10 or 100? x = {}", x);

    // formatted print
    // println!("{} days", 31);
    // println!("{0}, this is {1}. {1}, this is {0}", "Taufik", "Fauzi");
    // println!(
    //     "{subject} {verb} {object}",
    //     object = "the lazy dog",
    //     subject = "the quick brown fox",
    //     verb = "jumps over"
    // );
    // println!("Base 10:                  {}", 69420);
    // println!("Base 2 (binary):          {:b}", 69420);
    // println!("Base 8 (octal):           {:o}", 69420);
    // println!("Base 16 (hexadecimal):    {:x}", 69420);
    // println!("Base 16 (hexadecimal):    {:X}", 69420);
    // println!("{number:>5}", number = 1);

    // println!("My name is {0}, {1} {0}", "Bond", "James");

    // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct Structure(i32);
    // println!("This struct `{:?}` won't print...", Structure(3));
    // let pi = 3.141592;
    // println!("Pi is roughly {:.3}", pi)

    // #[derive(Debug)]
    // struct Structure(i32);

    // #[derive(Debug)]
    // struct Deep(Structure);

    // println!("{:?} months in a year", 12);
    // println!("Now {:?} will print!", Structure(3));
    // println!("Now {:?} will print!", Deep(Structure(7)));

    // #[derive(Debug)]
    // struct Person<'a> {
    //     name: &'a str,
    //     age: u8,
    // }

    // let name = "Taufik";
    // let age = 21;
    // let taufik = Person { name, age };

    // println!("{:#?}", taufik);

    // struct Structure(i32);
    // impl fmt::Display for Structure {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         write!(f, "{}", self.0)
    //     }
    // }

    // let minmax = MinMax(0, 14);

    // println!("Compare structures:");
    // println!("Display: {}", minmax);
    // println!("Debug: {:?}", minmax);

    // let big_range = MinMax(-300, 300);
    // let small_range = MinMax(-3, 3);

    // println!(
    //     "The big range is {big} and the small is {small}",
    //     small = small_range,
    //     big = big_range
    // );

    // let point = Point2D { x: 3.3, y: 7.2 };

    // println!("Compare points:");
    // println!("Display: {}", point);
    // println!("Debug: {:?}", point)

    // let complex = Complex {
    //     real: 3.3,
    //     imag: 7.2,
    // };

    // println!("Display: {}", complex);
    // println!("Debug: {:?}", complex);
}
