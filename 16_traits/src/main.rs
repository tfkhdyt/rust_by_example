// struct Sheep {
//     naked: bool,
//     name: &'static str,
// }

// trait Animal {
//     fn new(name: &'static str) -> Self;

//     fn name(&self) -> &'static str;
//     fn noise(&self) -> &'static str;

//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);
//             self.naked = true;
//         }
//     }
// }

// impl Animal for Sheep {
//     fn new(name: &'static str) -> Self {
//         Sheep {
//             naked: false,
//             name: name,
//         }
//     }

//     fn name(&self) -> &'static str {
//         self.name
//     }

//     fn noise(&self) -> &'static str {
//         if self.is_naked() {
//             "baaaaaah?"
//         } else {
//             "baaaaaah!"
//         }
//     }

//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//         Centimeters(inches as f64 * 2.54)
//     }
// }

// struct Seconds(i32);

use std::{io, iter, ops, vec::IntoIter};

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, rhs: Bar) -> Self::Output {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, rhs: Foo) -> Self::Output {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn parse_csv_document(src: impl io::BufRead) -> io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(",")
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|&&x| x > 0).map(|&x| x * 2)
}

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    // let mut dolly: Sheep = Animal::new("Dolly");
    // let mut dolly = Sheep::new("Dolly");

    // dolly.talk();
    // dolly.shear();
    // dolly.talk();

    // let one_second = Seconds(1);

    // // println!("One second looks like: {:?}", one_second);

    // // let this_is_true = (one_second == one_second);

    // let foot = Inches(12);

    // println!("One foot equals {:?}", foot);

    // let meter = Centimeters(100.0);
    // let cmp = if foot.to_centimeters() < meter {
    //     "smaller"
    // } else {
    //     "bigger"
    // };

    // println!("One foot is {} than one meter.", cmp);

    // let random_number = 0.234;
    // let animal = random_animal(random_number);

    // println!(
    //     "You've randomly chosen an animal, and it says {}",
    //     animal.noise()
    // );

    // println!("Foo + Bar = {:?}", Foo + Bar);
    // println!("Bar + Foo = {:?}", Bar + Foo);

    // let a = Droppable { name: "a" };
    // {
    //     let b = Droppable { name: "b" };
    //     {
    //         let c = Droppable { name: "c" };
    //         let d = Droppable { name: "d" };

    //         println!("Exiting block B");
    //     }
    //     println!("Just exited block B");
    //     println!("Exiting block A");
    // }
    // println!("Just exited block A");
    // // drop(a);

    // println!("end of the main function");

    // let mut sequence = 0..3;

    // println!("Four consecutive `next` calls on 0..3");
    // println!("> {:?}", sequence.next());
    // println!("> {:?}", sequence.next());
    // println!("> {:?}", sequence.next());
    // println!("> {:?}", sequence.next());

    // println!("Iterate through 0..3 using `for`");
    // for i in 0..3 {
    //     println!("> {}", i);
    // }

    // println!("The first four terms of the Fibonacci sequence are: ");
    // for i in fibonacci().take(4) {
    //     println!("> {}", i);
    // }

    // println!("The next four terms of the Fibonacci sequence are:");
    // for i in fibonacci().skip(4).take(4) {
    //     println!("> {}", i);
    // }

    // let array = [1u32, 3, 3, 7];

    // println!("Iterate the following array {:?}", &array);
    // for i in array.iter() {
    //     println!("> {}", i);
    // }

    // let v1 = vec![1, 2, 3];
    // let v2 = vec![4, 5];
    // let mut v3 = combine_vecs(v1, v2);

    // assert_eq!(Some(1), v3.next());
    // assert_eq!(Some(2), v3.next());
    // assert_eq!(Some(3), v3.next());
    // assert_eq!(Some(4), v3.next());
    // assert_eq!(Some(5), v3.next());

    // println!("all done");

    // let plus_one = make_adder_function(1);
    // assert_eq!(plus_one(2), 3);

    // let singles = vec![-3, -2, 2, 3];
    // let doubles = double_positives(&singles);

    // assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);

    // let unit = Unit;
    // let copied_unit = unit;

    // println!("original: {:?}", unit);
    // println!("copy    : {:?}", copied_unit);

    // let pair = Pair(Box::new(1), Box::new(2));
    // println!("original: {:?}", pair);

    // let moved_pair = pair;
    // println!("moved   : {:?}", moved_pair);

    // // println!("original: {:?}", pair);

    // let cloned_pair = moved_pair.clone();
    // drop(moved_pair);

    // // println!("copy: {:?}", moved_pair);

    // println!("clone   : {:?}", cloned_pair);

    let form = Form {
        username: "rustacean".to_owned(),
        age: 21,
    };

    // println!("{}", form.get());
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);

    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(21, age);
}
