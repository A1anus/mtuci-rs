fn main() {

    //     Обработка нескольких условий с помощью else if
    //         fn main() {
    //     let number = 6;
    //
    //     if number % 4 == 0 {
    //         println!("number is divisible by 4");
    //     } else if number % 3 == 0 {
    //         println!("number is divisible by 3");
    //     } else if number % 2 == 0 {
    //         println!("number is divisible by 2");
    //     } else {
    //         println!("number is not divisible by 4, 3, or 2");
    //     }
    // }
    
         // три вида циклов: loop, while и for. Давайте попробуем каждый из них.
    //     Пример бесконечного цикла
    //     fn main() {
    //     loop {
    //         println!("again!");
    //     }
    // }
    
    //     Возвращение значений из циклов
    //     fn main() {
    //     let mut counter = 0;
    //
    //     let result = loop {
    //         counter += 1;
    //
    //         if counter == 10 {
    //             break counter * 2;
    //         }
    //     };
    //
    //     println!("The result is {result}");
    // }
    
    //     Циклы с условием while
    //         fn main() {
    //     let mut number = 3;
    //
    //     while number != 0 {
    //         println!("{number}!");
    //
    //         number -= 1;
    //     }
    //
    //     println!("LIFTOFF!!!");
    // }
    //
    //     Цикл по элементам коллекции с помощью for
    //         fn main() {
    //     let a = [10, 20, 30, 40, 50];
    //     let mut index = 0;
    //
    //     while index < 5 {
    //         println!("the value is: {}", a[index]);
    //
    //         index += 1;
    //     }
    // }
    
    //     Перебор значений с помощью for
    //         fn main() {
    //     let a = [10, 20, 30, 40, 50];
    //
    //     for element in a {
    //         println!("the value is: {element}");
    //     }
    // }
    //
    //     пример for с использованием range
    
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }