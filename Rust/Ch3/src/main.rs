fn main() {
    const THREE: u32 = 3;
    println!("constant: {THREE}");

    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    
    //shadowing 

    let y = 8;

    let y = y +1;

    {
        let y = y*2;
        println!("the value of y is the inner scope is: {y}");
    }
    println!("the value of y is: {y}");


    // tuple example
    let tup: (i32,f64,u8) = (500,0.4,1);

    let (_a, b, c) = tup;

    let d = tup.0;

    println!("The value of tuple is: {d},{b},{c}");

    // array example
    let arr = [1,2,3,4,5];
    let arr2 = [3,2]; // creates an array of 2 elements being three 
    
    let arr = arr[0];
    let arr2 = arr2[0];

    println!("value of array 1 and 2 at index = 0: {arr}, {arr2}");

    //funtion example
    let par = 5; //parameter
    let res = another_function(par, 'h');
    println!("This is the result from the outside: {res}");

    //if statement example
    let val = 3;

    if val < 3{
        println!("val is less than 3");
    } else if val > 3 {
        println!("val is greater than 3");
    } else {
        println!("val is three")
    }

    // if statement with let
    let _val = if val == 3 {"3"} else {"not 3"}; //both results must have the same type 

    //loop example
    let mut  counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count =0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while loop example
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //for loop 
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }


    
}

fn another_function(x:i32, unit_label: char) -> i32 {
    println!("Im outside of Main !!!");
    println!("this is the number main gave me: {x}, {unit_label}");
    31 + 1 //returned value
}