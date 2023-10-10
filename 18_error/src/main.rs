#![allow(dead_code)]

use core::{fmt, num};
use std::{
    error::{self, Error},
    fmt::Display,
    num::ParseIntError,
};

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!");
}

// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         ah();
//     } else {
//         println!("Some refreshing {} is all I need.", beverage);
//     }
// }

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaaaaa!!!");
    }

    println!("I love {}s!!!", inside);
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u64,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

// #[derive(Debug)]
// enum Food {
//     Apple,
//     Carrot,
//     Potato,
// }

// #[derive(Debug)]
// struct Peeled(Food);

// #[derive(Debug)]
// struct Chopped(Food);

// #[derive(Debug)]
// struct Cooked(Food);

// fn peel(food: Option<Food>) -> Option<Peeled> {
//     match food {
//         Some(food) => Some(Peeled(food)),
//         None => None,
//     }
// }

// fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
//     match peeled {
//         Some(Peeled(food)) => Some(Chopped(food)),
//         None => None,
//     }
// }

// fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
//     chopped.map(|Chopped(food)| Cooked(food))
// }

// fn process(food: Option<Food>) -> Option<Cooked> {
//     food.map(|f| Peeled(f))
//         .map(|Peeled(f)| Chopped(f))
//         .map(|Chopped(f)| Cooked(f))
// }

// fn eat(food: Option<Cooked>) {
//     match food {
//         Some(food) => println!("Hmm, I love {:?}", food),
//         None => println!("Oh no! It wasn't edible."),
//     }
// }

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

// type AliasedResult<T> = Result<T, ParseIntError>;

// type Result<T> = std::result::Result<T, DoubleError>;

// #[derive(Debug)]
// enum DoubleError {
//     EmptyVec,
//     Parse(ParseIntError),
// }

// impl fmt::Display for DoubleError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match *self {
//             DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
//             DoubleError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
//         }
//     }
// }

// impl error::Error for DoubleError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match *self {
//             DoubleError::EmptyVec => None,
//             DoubleError::Parse(ref e) => Some(e),
//         }
//     }
// }

// impl From<ParseIntError> for DoubleError {
//     fn from(err: ParseIntError) -> Self {
//         DoubleError::Parse(err)
//     }
// }

// #[derive(Debug, Clone)]
// struct EmptyVec;

// impl Display for EmptyVec {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }

// impl Error for EmptyVec {}

fn main() {
    // drink("water");
    // drink("lemonade");
    // drink("still water");

    // let water = Some("water");
    // let lemonade = Some("lemonade");
    // let void = None;

    // give_adult(water);
    // give_adult(lemonade);
    // give_adult(void);

    // let coffee = Some("coffee");
    // let nothing = None;

    // drink(coffee);
    // drink(nothing);

    // let p = Person {
    //     job: Some(Job {
    //         phone_number: Some(PhoneNumber {
    //             area_code: Some(62),
    //             number: 895338865375,
    //         }),
    //     }),
    // };

    // assert_eq!(p.work_phone_area_code(), Some(62));

    // let apple = Some(Food::Apple);
    // let carrot = Some(Food::Carrot);
    // let potato: Option<Food> = None;

    // let cooked_apple = cook(chop(peel(apple)));
    // let cooked_carrot = cook(chop(peel(carrot)));
    // let cooked_potato = process(potato);

    // eat(cooked_apple);
    // eat(cooked_carrot);
    // eat(cooked_potato);

    // let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    // eat(cordon_bleu, Day::Monday);
    // eat(steak, Day::Tuesday);
    // eat(sushi, Day::Wednesday);

    // let apple = Some(Fruit::Apple);
    // let orange = Some(Fruit::Orange);
    // let no_fruit: Option<Fruit> = None;

    // let get_kiwi_as_fallback = || {
    //     println!("Providing kiwi as fallback");
    //     Some(Fruit::Kiwi)
    // };
    // let get_lemon_as_fallback = || {
    //     println!("Providing lemon as fallback");
    //     Some(Fruit::Lemon)
    // };

    // // let first_available_fruit = no_fruit.or(orange).or(apple);

    // let first_available_fruit = no_fruit
    //     .or_else(get_kiwi_as_fallback)
    //     .or_else(get_lemon_as_fallback);
    // println!("first_available_fruit: {:?}", first_available_fruit);

    // println!(
    //     "Variable apple was moved, so this line won't compile: {:?}",
    //     apple
    // );

    // let mut my_fruit: Option<Fruit> = None;
    // let get_lemon_as_fallback = || {
    //     println!("Providing lemon as fallback");
    //     Fruit::Lemon
    // };

    // let apple = Fruit::Apple;
    // let first_available_fruit = my_fruit.get_or_insert(apple);
    // let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    // println!("first_available_fruit is: {:?}", first_available_fruit);
    // println!("my_fruit is: {:?}", my_fruit);

    // let twenty = multiply("10", "2");
    // println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    // let number_str = "10";
    // let number = match number_str.parse::<i32>() {
    //     Ok(number) => number,
    //     Err(e) => return Err(e),
    // };
    // println!("{}", number);
    // Ok(())

    // let twenty = multiply("10", "2");
    // print(twenty);

    // let tt = multiply("t", "2");
    // print(tt);

    // let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];

    // print(double_first(numbers));
    // print(double_first(empty));
    // print(double_first(strings));

    let strings = vec!["tofu", "93", "18"];
    // let mut errors = vec![];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<u8>())
    //     .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
    //     .collect();
    // let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();

    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

    println!("Results: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

// fn double_first(vec: Vec<&str>) -> Result<i32> {
//     // let first = vec.first().unwrap();
//     // 2 * first.parse::<i32>().unwrap()

//     // let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

//     // opt.map_or(Ok(None), |r| r.map(Some))

//     // vec.first()
//     //     .ok_or(DoubleError)
//     //     .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))

//     // vec.first()
//     //     .ok_or_else(|| EmptyVec.into())
//     //     .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))

//     let first = vec.first().ok_or(DoubleError::EmptyVec)?;
//     let parsed = first.parse::<i32>()?;
//     Ok(2 * parsed)
// }

// fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
//     // match first_number_str.parse::<i32>() {
//     //     Ok(first_number) => match second_number_str.parse::<i32>() {
//     //         Ok(second_number) => Ok(first_number * second_number),
//     //         Err(e) => Err(e),
//     //     },
//     //     Err(e) => Err(e),
//     // }

//     // first_number_str.parse::<i32>().and_then(|first_number| {
//     //     second_number_str
//     //         .parse::<i32>()
//     //         .map(|second_number| first_number * second_number)
//     // })

//     let first_number = first_number_str.parse::<i32>()?;
//     let second_number = second_number_str.parse::<i32>()?;

//     Ok(first_number * second_number)
// }

// fn print(result: Result<i32>) {
//     match result {
//         Ok(n) => println!("The first doubled is {}", n),
//         Err(e) => {
//             println!("Error: {}", e);
//             if let Some(source) = e.source() {
//                 println!(" Caused by: {}", source);
//             }
//         }
//     }
// }
