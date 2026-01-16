fn add(a: i32, b: i32) -> i32{
    return a+b
}
fn subtract(a: i32, b: i32) -> i32{
    if a>=b{
        return a-b;
    }
    else{
        return b-a;
    }
   
}

fn main(){
    let sum = add(5, 3);
    let difference = subtract(5, 10);
    println!("Sum: {}, Difference: {}", sum, difference);
}