#![allow(dead_code)]

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// struct Unit;

// struct Pair(i32, f32);

// #[derive(Debug)]
// struct Point {
//     x: f32,
//     y: f32,
// }

// #[derive(Debug)]
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }

fn main() {
    // let name = String::from("Taufik");
    // let age = 21;
    // let taufik = Person { name, age };

    // println!("{:?}", taufik);

    // let point = Point { x: 10.3, y: 0.4 };
    // println!("point coordinates: ({}, {})", point.x, point.y);

    // let bottom_right = Point { x: 5.2, ..point };
    // println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // let Point {
    //     x: left_edge,
    //     y: top_edge,
    // } = point;

    // let _rectangle = Rectangle {
    //     top_left: Point {
    //         x: left_edge,
    //         y: top_edge,
    //     },
    //     bottom_right: bottom_right,
    // };

    // let _unit = Unit;
    // let pair = Pair(1, 0.1);
    // println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // let Pair(integer, decimal) = pair;
    // println!("pair contains {:?} and {:?}", integer, decimal);

    // println!("Rectangle area: {}", rect_area(_rectangle));
    // println!("Square: {:#?}", square(Point { x: 5.0, y: 5.0 }, 6.0));

    // let pressed = WebEvent::KeyPress('x');
    // let pasted = WebEvent::Paste("my text".to_owned());
    // let click = WebEvent::Click { x: 20, y: 80 };
    // let load = WebEvent::PageLoad;
    // let unload = WebEvent::PageUnload;

    // inspect(pressed);
    // inspect(pasted);
    // inspect(click);
    // inspect(load);
    // inspect(unload);

    // let x = Operations::Add;
    // println!("{}", x.run(6, 9))

    // use crate::Status::{Poor, Rich};
    // use crate::Work::*;

    // let status = Poor;
    // let work = Civilian;

    // match status {
    //     Rich => println!("The rich have lots of money!"),
    //     Poor => println!("The poor have no money..."),
    // }

    // match work {
    //     Civilian => println!("Civilians work!"),
    //     Soldier => println!("Soldiers fight!"),
    // }

    // println!("zero is {}", Number::Zero as i32);
    // println!("one is {}", Number::One as i32);

    // println!("roses are #{:06x}", Color::Red as i32);
    // println!("violets are #{:06x}", Color::Blue as i32)

    // let mut list = List::new();
    // list = list.prepend(1);
    // list = list.prepend(2);
    // list = list.prepend(3);

    // println!("linked list has length: {}", list.len());
    // println!("{}", list.stringify());

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // THRESHOLD = 5;
}

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

// enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }

// impl List {
//     fn new() -> List {
//         Self::Nil
//     }

//     fn prepend(self, elem: u32) -> List {
//         Self::Cons(elem, Box::new(self))
//     }

//     fn len(&self) -> u32 {
//         match *self {
//             Self::Cons(_, ref tail) => 1 + tail.len(),
//             Self::Nil => 0,
//         }
//     }

//     fn stringify(&self) -> String {
//         match *self {
//             Self::Cons(head, ref tail) => {
//                 format!("{}, {}", head, tail.stringify())
//             }
//             Self::Nil => format!("Nil"),
//         }
//     }
// }

// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }

// enum Status {
//     Rich,
//     Poor,
// }

// enum Work {
//     Civilian,
//     Soldier,
// }

// enum VeryVerbosEnumOfThingsToDoWithNumbers {
//     Add,
//     Substract,
// }

// type Operations = VeryVerbosEnumOfThingsToDoWithNumbers;

// impl VeryVerbosEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             Self::Add => x + y,
//             Self::Substract => x - y,
//         }
//     }
// }

// enum WebEvent {
//     PageLoad,
//     PageUnload,
//     KeyPress(char),
//     Paste(String),
//     Click { x: i64, y: i64 },
// }

// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
//         WebEvent::Paste(s) => println!("pasted \"{}\".", s),
//         WebEvent::Click { x, y } => {
//             println!("clicked at x={}, y={}.", x, y);
//         }
//     }
// }

// fn rect_area(rectangle: Rectangle) -> f32 {
//     let Rectangle {
//         top_left: Point { x, y },
//         bottom_right: _,
//     } = rectangle;

//     x * y
// }

// fn square(point: Point, f: f32) -> Rectangle {
//     Rectangle {
//         top_left: point,
//         bottom_right: Point { x: f, y: f },
//     }
// }
