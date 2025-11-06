fn main() {
    println!("Hello, chandu champion!");
}

fn main() { //scope is function
    let x = 5;
    println!("x is {}",x);
}

fn main() { //overdie if you dont want to make it mutable
    let x = 5;
    println!("x is {}",x);
    let x = 6; // or x=x+1
    println!("x is {}",x);
}

fn main() { //overdie if you dont want to make it mutable
    let x = 5;
    println!("x is {}",x);
    {
        let x = 7;    // creating a seperate scope for x, the x here is a different variable from the outer x
        println!("x is {}",x);
    }
    let x = x+5; // or x=x+1 // this will change the value of x in the outer scope
    println!("x is {}", x);
}

fn main() {
    //shadowing
    let x = 5;
    println!("x is {}", x);
    {
        let x = x - 2;
        println!("x is {}", x);
    };
    let x = x + 3;
    println!("x is {}", x);
}

fn main() {
    //shadowing
    let mut  x = 5;
    println!("x is {}", x);
    x = 7;
    println!("x is {}", x);
}

fn main() { //constants are immutable by default
    const SECONDS_IN_MINUTE: u32 = 60;
    // const SECONDS_IN_MINUTE: u32 = 70; // this will give an error because const cannot be changed
    // SECONDS_IN_MINUTE = 70; // this will give an error because const cannot be changed
    println!("Seconds in a minute: {}", SECONDS_IN_MINUTE);
}