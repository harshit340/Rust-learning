/* Identify duplicate code.
Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
Update the two instances of duplicated code to call the function instead. */

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { // this error will be resolve by restricting the type of T because T can be anything so we have to restrict
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}