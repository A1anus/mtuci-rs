fn main() {

    //      let s1 = String::from("hello");
    //
    //     let len = calculate_length(&s1);
    //
    //     println!("The length of '{}' is {}.", s1, len);
    // }
    
    // let mut s = String::from("hello");
    //
    // change(&mut s);
    
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s; // мы не можем заимствовать s как изменяемые более одного раза в один момент
    //
    // println!("{}, {}", r1, r2);
    
    // никакой ошибки, так как ссылки находятьсся в разных облостях видимости
    // let mut s = String::from("hello");
    //
    // {
    // let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    //
    // let r2 = &mut s;
    
            let string = no_dangle();
    }
    
    //     Этот код приводит к ошибке:
    //
    // fn main() {
    //     let mut s = String::from("hello");
    //
    //     let r1 = &s; // no problem
    //     let r2 = &s; // no problem
    //     let r3 = &mut s; // BIG PROBLEM
    //
    //     println!("{}, {}, and {}", r1, r2, r3);
    // }
    // У нас также не может быть изменяемой ссылки, пока у нас есть неизменяемая ссылка на то же значение.
    
    fn no_dangle() -> String {
        let s = String::from("hello");
    
        s
    }