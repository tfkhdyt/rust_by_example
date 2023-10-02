use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}", self.real, self.imag)
    }
}

fn main() {
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

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
