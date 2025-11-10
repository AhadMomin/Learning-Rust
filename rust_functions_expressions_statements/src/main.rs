//expressions:
// statements:
// statements are used to perform actions
// expressions are used to return a value (true or false)

fn test_one(x:i32,y:i32){
    let z = x + y;
    return println!("z is {}", z);
}


fn main() {
    println!("Hello, world!");
    test_one(1,1);
    expression_statement();
    let number = { //this is a block that returns a value so no semicolon is needed. Semicolon will get a errors.
        let x = 3;
        x + 1 //this is an expression that returns a value so no semicolon is needed. Semicolon will get a errors.
    };
    println!("number is {}", number);
    let result = add_numbers_return2(15,5);
    println!("result is {}", result);
    // add_numbers_return2(1,2);
}

fn expression_statement(){
    let x = 2<3;
    println!("x is {}", x);

}

fn add_numbers_return(x:i32,y:i32)->i32{
     x + y //expression after function definition is returned
     //or 
     //return x + y; //allowed to add semicolon or not
}

fn add_numbers_return2(x:i32,y:i32)->i32{
    let result = x + y; //allowed to add semicolon or not
    if 
        result > 10 {
        return result -10;
    }
    result
}