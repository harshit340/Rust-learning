struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
}

// creating a new user 
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // creating the instance from another instance
     let mut user2 = User {
            active: user1.active,
            username: String::from("harshit here"),
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };

    // creating the instance from another instance using The syntax ..

    let user3 = User {
        active: user2.active,
        username: user2.username.clone(), // we are writing this becoz String does not implement the Copy trait, it cannot be moved more than once
        ..user2
    };
    

    println!("print username2:{}",user1.username);
    println!("print username2:{}",user2.username);
    println!("print username2:{}",user3.username);
}