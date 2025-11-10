//conditional statements
fn main() {
    println!("Hello, world!");
    statement1();
    statement2();
    compound_conditions();
    if_statement();
}


//operators:
// > greater than
// < less than
// >= greater than or equal to
// <= less than or equal to
// == equal to
// != not equal to
// && and
// || or
// ! not
// ? ternary operator
// : ternary operator
// ?: ternary operator

fn statement1(){
    let cond =2 < 3; //true
    println!("{}",cond);
}

fn statement2(){
    let cond = (2 as f32) < 2.2;
    println!("{}",cond);
}

//compound conditions: && and || or ! not
 
fn compound_conditions(){

    let cond = (2 as f32) < 2.2;

    let cond2 = true || cond;
    let cond3 = !(true || !cond); //not operator is used to negate the condition

    println!("{}",cond2);
    println!("{}",cond3);
}

fn if_statement(){

    let food = "cookie";

    if food == "cookie"{
        println!("I like cookies");
    } else if food == "apple"{
        println!("I like apples");
    } else {
        println!("I don't like {}", food);
    }
}