fn main() {
    
    //strings 
    let mut s = String::from("hello");

    s.push_str(", world");

    let y = s.clone();//if this wasnt here s would no longer be acessable since it is being borrowed


    

    change(&mut s);

    println!("{},{}",s,y);

    //slices 

    let c = String::from("hello world");

    let _hello = &s[..5]; //could also be [0..5]
    let _world = &s[6..]; //could also be [6..11]
    let _entire = &s[..];

    let fw = first_word(&c);
    println!("{}",fw);


}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}