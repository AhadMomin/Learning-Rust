fn main() { 
    //let x: u8 = 12;
    //let y: i8 = 10;
    //rintln!("x is {}", x);
   // println!("y is {}", y);
    //println!("Hello, world!");

    //let z = x + y; // this will give an error because x and y are different types
    //println!("z is {}", z);
    //arthematic_operations1();
    //arthematic_operations2();
//arthematic_operations3();
    //arthematic_operations4();
    //arthenatic_operations5();
    //typecasting1();
    //typecasting2();
    //typecasting3();
    //typecasting4();
    //typecasting5();
    string_to_int();
}


// fn arthematic_operations1() {
//     let x: i8 = 127; // this will give an error because 256 is out of range for u8
//     let y: i8 = -128;
//     let z = x + y;
//     println!("z is {}", z);

// }

// fn arthematic_operations2() {
//     let x: f32 = 1.23;
//     let y: f32 = 4.56;
//     let z = x + y;
//     println!("z is {}", z);
// }

// fn arthematic_operations3() {//overflow and underflow
//     let x: u8 = 255;
//     let y: u8 = 1;
//     let z = x + y;
//     println!("z is {}", z); //this will give 0 because 255 + 1 = 256 which is out of range for u8
//     let x: i8 = 127;
//     let y: i8 = -128;
//     let z = x + y;
//     println!("z is {}", z); //this will give 0 because 127 + -128 = -1 which is out of range for i8
// }

// fn arthematic_operations4() {//this will not give an error the division will be performed and the result will be truncated
//     let x: u8 = 128;
//     let y: u8 = 10;

//     let z = x/y;
//     println!{"Z is {}", z}
// }

// fn arthenatic_operations5() {
//     let x: f32 = 128.0;
//     let y: f32 = 10.0;
//     let z = x/y;
//     let k = x*y;
//     let m = x%y; //mod operation
//     println!("z is {}", z);
//     println!("k is {}", k);
//     println!("m is {}", m);
// }

// //typecasting

// fn typecasting1() {
//     let x = 255 as u8;
//     let y = x as u8
//     println!("x is {}", x);
//     println!("y is {}", y);
// }

// fn typecasting2() {
//     let x = 255.0_f32;
//     let y = 0.0_f32;
//     let z = x/y;
//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// fn typecasting3() {
//     let x = 255i64;
//     let y = 0i64;
//     let z = x*y;
//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// fn typecasting4() {
//     let x = 255i64;
//     let y = 1i32;
//     let z = x*(y as i64); //this will not give an error because we are casting y to i64
//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// fn typecasting5() { //this will give an error because casting from 64 to 32 will overflow the data
//     let x = (i64::MAX as i64) + 1; // called max value of i64 and added 1 to it
//     let y = 10_i32;

//     let z = x as i32 / y;
//     println!("x is {}", z)
// }

use std::io;

fn string_to_int() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading line");
    
    let int_input : i64 = input.trim().parse().unwrap();

    println!("you entered: {}", int_input+2);
}

//parse() is a method that converts a string to a number
//trim() is a method that removes whitespace from the beginning and end of a string
//unwrap() is a method that returns the value of the result if it is Ok, otherwise it will panic