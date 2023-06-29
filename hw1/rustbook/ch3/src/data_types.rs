use std::io;

fn main() {

//Числа с плавающей запятой:
//     fn main() {
//     let x = 2.0; // f64
//
//     let y: f32 = 3.0; // f32
// }

//Числовые операции:
// fn main() {
//     // addition
//     let sum = 5 + 10;
//
//     // subtraction
//     let difference = 95.5 - 4.3;
//
//     // multiplication
//     let product = 4 * 30;
//
//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1
//
//     // remainder
//     let remainder = 43 % 5;
// }

//     Логический тип данных
//
//     fn main() {
//     let t = true;
//
//     let f: bool = false; // with explicit type annotation
// }

//     Символьный тип данных
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

//     Кортежи
//     fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//
//     let five_hundred = x.0;
//
//     let six_point_four = x.1;
//
//     let one = x.2;
// }

    // Массивы
    // код, вызывает панику при вводе числа больше или равно 5, так как длинна массива всего 5

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}