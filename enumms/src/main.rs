/* enums give you a way of saying a value is one of a possible set of values.*/

// Define an enum to represent different kinds of IP addresses
#[derive(Debug)]  // This trait is used to enable formatting of a type using the {:?} formatter in macros like println!
enum IpAddrKind {
    V4,
    V6,
}
// Define a struct IpAddr that holds an IpAddrKind and an address string
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main(){
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home kind {:?} and address {}",home.kind , home.address);
    println!("loopback kind {:?} and address {}",loopback.kind ,loopback.address);
}



// option in enum 

/* Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T> */

enum Option<T>{
    None,
    Some(T),
}

/* The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly
 */

 let absent_number: Option<i32> = None; // we can directly use options like this no need to use it as above
