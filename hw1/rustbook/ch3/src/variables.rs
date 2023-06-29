fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x); получили сообщение об ошибке
    // cannot assign twice to immutable variable x``, потому что попытались
    // присвоить новое значение неизменяемой переменной x.
         let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }
    
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; - пример объявления константы
    