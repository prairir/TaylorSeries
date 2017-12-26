fn factorial(num: i32) -> i32{
    if num==0{
        return 1;
    }
    else{
        return num * factorial(num -1)
    }
}

fn main() {
    let num = factorial(5);
    println!("{}", num);
}
