// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// // impl From<i32> for Number {
// //     fn from(value: i32) -> Self {
// //         Number { value: value }
// //     }
// // }

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// #[derive(Debug, PartialEq)]
// struct EvenNumber(i32);

// impl TryFrom<i32> for EvenNumber {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNumber(value))
//         } else {
//             Err(())
//         }
//     }
// }

use std::fmt::Display;

struct Circle {
    radius: i32,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // let my_str = "hello";
    // let my_string = String::from(my_str);

    // let num = Number::from(30);
    // println!("My number is {:?}", num);

    // let int = 5;
    // let num: Number = int.into();
    // println!("My number is {:?}", num);

    // assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    // assert_eq!(EvenNumber::try_from(5), Err(()));

    // let results: Result<EvenNumber, ()> = 8i32.try_into();
    // assert_eq!(results, Ok(EvenNumber(8)));
    // let results: Result<EvenNumber, ()> = 5i32.try_into();
    // assert_eq!(results, Err(()));

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
