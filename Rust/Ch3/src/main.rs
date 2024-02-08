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

    let tup: (i32,f64,u8) = (500,0.4,1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    
}
