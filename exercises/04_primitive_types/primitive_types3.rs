fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a = [2; 100];

    if a.len() >= 100 {
        println!("{}",a[2]);
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
