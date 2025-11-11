fn main() {
    println!("Hello, world!");
}


## Ram management (Random Access Memory)
- RAM is a short-term memory that is used to store data that is being processed.
- RAM is volatile, meaning that it is lost when the computer is turned off.
- RAM is used to store data that is being processed.

# Stack management
- Stack is a short-term memory that is used to store data that is being processed.
- Stack is not volatile, meaning that it is not lost when the computer is turned off.
- Stack is used to store data that is being processed.
- FILO (First In Last Out)
- fixed size is what stack is stored in, it is a fixed size memory allocation

# Heap management
- Heap is a long-term memory that is used to store data that is not being processed.
- Heap is not volatile, meaning that it is not lost when the computer is turned off.
- Heap is used to store data that is not being processed.
- for heap, the memory allocation is dynamic, it is not a fixed size memory allocation

fn stack_management(){
    let x = 5;
    let y = 6;
    let z = x + y;
    println!("z is {}", z);
}

fn heap_management(){//slow performance because of the pointer to the heap memory allocation where the data is stored 
    let x = Box::new(5); //pointer to the heap memory allocation
    let y = Box::new(6); //pointer to the memory allocation
    let z = x + y; //pointer to the memory allocation
    println!("z is {}", z); //pointer to the memory allocation
}

fn main(){
    stack_management();
    heap_management();
}