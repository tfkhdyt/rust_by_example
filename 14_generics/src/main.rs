// use std::fmt::{Debug, Display};

// struct A;
// struct S(A);
// struct SGen<T>(T);
// struct GenericVal<T>(T);
// struct Val {
//     val: f64,
// }
// struct GenVal<T> {
//     gen_val: T,
// }
// struct Empty;
// struct Null;

// trait DoubleDrop<T> {
//     fn double_drop(self, _: T);
// }

// impl<T, U> DoubleDrop<T> for U {
//     fn double_drop(self, _: T) {}
// }

// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }

// impl<T> GenVal<T> {
//     fn value(&self) -> &T {
//         &self.gen_val
//     }
// }

// impl GenericVal<f32> {}
// impl GenericVal<S> {}
// impl<T> GenericVal<T> {}

// fn reg_fn(_s: S) {}
// fn gen_spec_t(_s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>) {}
// fn generic<T>(_s: SGen<T>) {}

// fn printer<T: Display>(t: T) {
//     println!("{}", t);
// }

// struct Sv2<T: Display>(T);

// trait HasArea {
//     fn area(&self) -> f64;
// }

// impl HasArea for Rectangle {
//     fn area(&self) -> f64 {
//         self.length * self.height
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     length: f64,
//     height: f64,
// }
// struct Triangle {
//     length: f64,
//     height: f64,
// }

// fn print_debug<T: Debug>(T: &T) {
//     println!("{:?}", t);
// }

// fn area<T: HasArea>(t: T) {
//     println!("{:?}", t);
// }

use std::{marker::PhantomData, ops};

fn main() {
    // let _s = S(A);
    // let _char: SGen<char> = SGen('a');

    // let _t = SGen(A);
    // let _i32 = SGen(6);
    // let _char = SGen('a');

    // reg_fn(S(A));
    // gen_spec_t(SGen(A));
    // gen_spec_i32(SGen(6));

    // generic::<char>(SGen('a'));
    // generic(SGen('c'));

    // let x = Val { val: 3.0 };
    // let y = GenVal { gen_val: 3i32 };
    // println!("{}, {}", x.value(), y.value());

    // let empty = Empty;
    // let null = Null;

    // empty.double_drop(null);

    // let s = S(vec![1]);

    // let rectangle = Rectangle{length: 3.0, 4.0};
    // let _triangle = Triangle{length: 3.6, 4.0};

    // print_debug(&rectangle);
    // println!("Area: {}", area(&rectangle));

    // let cardinal = Cardinal;
    // let blue_jay = BlueJay;
    // let _turkey = Turkey;

    // println!("A cardinal is {}", red(&cardinal));
    // println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));

    // let string = "words";
    // let array = [1, 2, 3];
    // let vec = vec![1, 2, 3];

    // compare_prints(&string);
    // // compare_prints(&array);
    // compare_types(&array, &vec);

    // let vec = vec![1, 2, 3];
    // vec.print_in_option();

    // let age = Years(21);
    // let age_days = age.to_days();

    // println!("Old enough {}", old_enough(&age));
    // println!("Old enough {}", old_enough(&age_days.to_years()));
    // // println!("Old enough {}", old_enough(&age_days));

    // let years = Years(42);
    // let years_as_primitive_1: i64 = years.0;
    // let Years(years_as_primitive_2) = years;

    // let number_1 = 3;
    // let number_2 = 10;
    // let container = Container(number_1, number_2);

    // println!(
    //     "Does container contain {} and {}: {}",
    //     &number_1,
    //     &number_2,
    //     container.contains(&number_1, &number_2)
    // );
    // println!("First number: {}", container.first());
    // println!("Last number: {}", container.last());
    // println!("The difference is: {}", difference(&container));

    // let tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // let tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    // let struct1: PhantomStruct<char, f32> = PhantomStruct {
    //     first: 'Q',
    //     phantom: PhantomData,
    // };
    // let struct2: PhantomStruct<char, f64> = PhantomStruct {
    //     first: 'Q',
    //     phantom: PhantomData,
    // };

    // println!("tuple1 == tuple2 yields: {}", tuple1 == tuple2);
    // println!("struct1 == struct2 yields: {}", struct1 == struct2);

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Clone, Copy)]
enum Mm {}

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);
impl<Unit> ops::Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

// #[derive(PartialEq)]
// struct PhantomTuple<A, B>(A, PhantomData<B>);

// #[derive(PartialEq)]
// struct PhantomStruct<A, B> {
//     first: A,
//     phantom: PhantomData<B>,
// }

// struct Container(i32, i32);

// trait Contains {
//     type A;
//     type B;

//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains for Container {
//     type A = i32;
//     type B = i32;

//     fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }

//     fn first(&self) -> i32 {
//         self.0
//     }

//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// fn difference<C: Contains>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// struct Years(i64);
// struct Days(i64);

// impl Years {
//     pub fn to_days(&self) -> Days {
//         Days(self.0 * 365)
//     }
// }

// impl Days {
//     pub fn to_years(&self) -> Years {
//         Years(self.0 / 365)
//     }
// }

// fn old_enough(age: &Years) -> bool {
//     age.0 >= 18
// }

// trait PrintInOption {
//     fn print_in_option(self);
// }

// impl<T> PrintInOption for T
// where
//     Option<T>: Debug,
// {
//     fn print_in_option(self) {
//         println!("{:?}", Some(self));
//     }
// }

// fn compare_prints<T: Debug + Display>(t: &T) {
//     println!("Debug: {:?}", t);
//     println!("Display: {}", t);
// }

// fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
//     println!("t: `{:?}`", t);
//     println!("u: `{:?}`", u);
// }

// struct Cardinal;
// struct BlueJay;
// struct Turkey;

// trait Red {}
// trait Blue {}

// impl Red for Cardinal {}
// impl Blue for BlueJay {}

// fn red<T: Red>(_: &T) -> &'static str {
//     "red"
// }
// fn blue<T: Blue>(_: &T) -> &'static str {
//     "blue"
// }
