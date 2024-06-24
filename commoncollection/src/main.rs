

///////////////////////////////////////////////////////////////////////////////////////////
/// about vectors 

// commet out below to see

/* fn main(){
    let a = [1,2,3];
    let mut v:Vec<i32> = Vec::new();  // creating vector
    v.push(1);
    v.push(2);
    v.push(3);

    // initializing the value of vector while creating a vector

    let v2:Vec<i32> =vec![1,2,3];
    //acesss the element of vector
    let third =&v2[2];
    println!("The third element is {}",third);

    //another way of accessing the vector elements amd some time you may go out of size of vector and trying to access the element which leads to the crase of program so we use this type

    match v.get(2){
        Some(third)=> println!("The third element is {}",third),
        None => println!("There is no third element"),
    }

    // to iterate over all the elements

    for i in &v2{
        println!("{}",i);
    }

  // if you want to store the different type of data in vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1]{
        SpreadsheetCell::Int(i)=>println!("{}",i),
        _ => println!("Not a integer!")
    }

}
 */


 /////////////////////////////////////////////////////////////////////////////////////////////////////
 /// learning String
 
/*  fn main(){
      // Strings are stored as a collection of UTF-8 encoded bytes
       
       // methods of initailizing the string

       let s1 = String::new();
       let s2 ="initial contents";
       let s3 = s2.to_string();
       let s4 = String::from("initial contents");

       // append the string

       let mut s = String::from("Harshit");
       s.push_str("bar");
       s.push("!");
       // we also append the string using the + operator

      let s5:String = s2+&s;
      

 }
 */


 
