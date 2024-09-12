use std::io;

fn main() {

    // Variables and mutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("\n------------\n");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of the constant is {THREE_HOURS_IN_SECONDS}");
    println!("\n------------\n");
    
    // Shadowing
    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");
    println!("\n------------\n");

    /*
        SPACES:

        First:
        The first create a new variable, so it doesn't
        presents an type error

        Second:
        The second one presents a type error because
        we defined the spaces as a &str and then we are assingning
        a usize  
    */
    let spaces =  "     ";
    let spaces = spaces.len();

    println!("Value of spaces is: {spaces}");
    
    //let mut spaces= "     ";
    //spaces = spaces.len();
    //println!("Value of mutable spaces is: {spaces}");
    
    println!("\n------------\n");
    
    // This shows another way to declare variables
    let number = 57_u8;
    println!("Value of number is {number} ");
    
    println!("\n------------\n");
    
    // The touple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    
    // Destructuring
    let (x1, y1, z1) = tup2;
    
    println!("The value of y is: {y1}");
    
    // Accessing
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    
    println!("Five hundred: {five_hundred}");
    println!("Six point four: {six_point_four}");
    println!("One: {one}");
    
    println!("\n------------\n");

    // The array type
    
    let array_a = [1,2,3,4,5];

    // Usearray when you know the numbers of elements
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]; 

    // You can add the type
    let array_b: [i32; 5] = [1, 2, 3, 4, 5];

    let array_c = [3; 5];

    let first = array_a[0];
    let second = array_a[1];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = array_b[index];

    println!("The value of the element at index {index} is {element}");
} 
