use std::io;

fn main(){
    println!("enter the string");
    let mut w = String::new();
    io::stdin()
        .read_line(&mut w)   // read user input
        .expect("Failed to read line");
    let ans = first_word(&w);

    println!("your first letter is {}",ans);
}


fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}