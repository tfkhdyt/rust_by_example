// #[derive(Debug)]
// struct Matrix(f32, f32, f32, f32);

// impl fmt::Display for Matrix {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "( {} {} )
// ( {} {} )",
//             self.0, self.1, self.2, self.3
//         )
//     }
// }

use std::mem;

fn main() {
    // let logical = true;

    // let a_float = 1.0;
    // let an_integer = 5i32;

    // let default_float = 3.0;
    // let default_integer = 7;

    // let mut inferred_type = 12;
    // inferred_type = 4294967296i64;

    // let mut mutable = 12;
    // mutable = 21;

    // // mutable = true;

    // let mutable = true;

    // println!("1 + 2 = {}", 1u32 + 2);
    // println!("1 - 2 = {}", 1i32 - 2);
    // println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    // println!("true AND false is {}", true && false);
    // println!("true OR false is {}", true || false);
    // println!("NOT true is {}", !true);
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {:04b}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    // println!("One million is written as {}", 1_000_000u32);

    // let long_tuple = (
    //     1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    // );

    // println!("Long tuple first value: {}", long_tuple.0);
    // println!("Long tuple second value: {}", long_tuple.1);

    // let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // println!("tuple of tuples: {:?}", tuple_of_tuples);

    // let pair = (1, true);
    // println!("Pair is {:?}", pair);
    // println!("The reversed pair is {:?}", reverse(pair));

    // println!("One element tuple: {:?}", (5u32,));
    // println!("Just an integer: {:?}", (5u32));

    // let tuple = (1, "hello", 4.5, true);
    // let (a, b, c, d) = tuple;
    // println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // // println!("{:?}", matrix)
    // println!("Matrix:\n{}", matrix);
    // println!("Transpose:\n{}", transpose(matrix));

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of array: {}", xs[0]);
    println!("Second element of array: {}", xs[1]);

    println!("Number of elements in array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

// fn reverse(pair: (i32, bool)) -> (bool, i32) {
//     let (int_param, bool_param) = pair;
//     (bool_param, int_param)
// }

// fn transpose(matrix: Matrix) -> Matrix {
//     Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
// }
