fn main() {
    println!("Hello, world!");
    let x: i32 = 5;
    println!("x is {}", x);
    float();
    bool();
    char();
    tuple();
    array();
}

fn float() {
    let x: f32 = 5.90; //single precision
    let y: f64 = 5.12; //double precision
    println!("x is {}", x);
    println!("y is {}", y);
}

fn bool() {
    let boolean: bool = true;
    println!("boolean is {}", boolean);
    let boolean2: bool = false;
    println!("boolean2 is {}", boolean2);
}

fn char() {
    let character: char = 'a';
    println!("character is {}", character);
    let character2: char = '9';
    println!("character2 is {}", character2);
}

fn tuple() {
    let tup: (i32, bool, char) = (1, true, 'a');
    let mut tup2: (i32, bool, char) = (2, false, 'b');
    println!("({},{},{})",tup2.0,tup2.1,tup2.2);
    println!("{}",tup.0);
    println!("tuple is {:?}", tup);
    tup2.0 = 3;
    println!("{}",tup2.0);
    tup2 = (4, true, 'c');
    println!("{:?}",tup2);
}

fn array() {
    let mut arr : [i32;5] = [1,2,3,4,5];
    let arr2 : [i32;0] = []; //empty array
    arr[4] = 6;
    println!("{}", arr[4]);
    println!("{:?}", arr); 
    println!("{:?}", arr2);}
//i8, i16, i32, i64, i128, isize (signed integers // signed integers are positive and negative integers)
//u8, u16, u32, u64, u128, usize (unsigned integers // unsigned integers are positive integers)
//f32, f64 (floating point numbers, single precision and double precision)
//bool (true or false) or 1 or 0
//char (single character) 'a', 'b', 'c', '9', '$' single quotes are used to denote a character
//string "Hello, world!" //double quotes are used to denote a string
//array [1,2,3,4,5] //fixed size 
//vector 
//hashmap //key-value pairs
//set //unique values
//tuple (a fixed size collection of values) (1, 2, 3) (1, 2, 3) {1,true, 'a'} 

//u8 0-255 (stored in 8 bits) 2^8-1 = 255
//i8 -128-127 (stored in 8 bits) 2^8-1 = 127
//u16 0-65535 (stored in 16 bits) 2^16-1 = 65535
//i16 -32768-32767 (stored in 16 bits) 2^16-1 = 32767
//u32 0-4294967295 (stored in 32 bits) 2^32-1 = 4294967295
//i32 -2147483648-2147483647 (stored in 32 bits) 2^32-1 = 2147483647
//u64 0-18446744073709551615 (stored in 64 bits) 2^64-1 = 18446744073709551615
//i64 -9223372036854775808-9223372036854775807 (stored in 64 bits) 2^64-1 = 9223372036854775807
//u128 0-340282366920938463463374607431768211455 (stored in 128 bits) 2^128-1 = 340282366920938463463374607431768211455
//i128 -170141183460469231731687303715884105728-170141183460469231731687303715884105727 (stored in 128 bits)
//usize 0-18446744073709551615 (stored in 64 bits) 2^64-1 = 18446744073709551615
//isize -9223372036854775808-9223372036854775807 (stored in 64 bits) 2^64-1 = 9223372036854775807
