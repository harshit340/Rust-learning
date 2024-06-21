/* fn main() {
    let x : i32 = 45;  //this can be neg and positive both
    let y : u32 = 23;  //this can be positive only

    println!("x:{},y:{}",x,y);
}  */


// boolean
/* 
fn main() {
    let is_male = false;
    let is_above_18 = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
} */


// strings 

/* fn main(){
    let greeting = String::from("hello world");
    println!("{}",greeting);

    let char1 = greeting.chars().nth(1);
    println!("char1: {}",char1.unwrap());  // this will give an error when nth(1000) but this will give an correct answer for nth(1) bcoz it is define
} */


fn main(){
    let sentence = String::from("my name is harshit");
    let first_word = get_first_word(sentence);
    print!("First word is : {}",first_word);
}

fn get_first_word(sentence:String)->String{
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());

        if char == ' '{
            break;
        } 
    }
    return ans;
}