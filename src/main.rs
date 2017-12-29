fn factorial(num: i32) -> i32{
    if num==0{
        return 1;
    }
    else{
        return num * factorial(num -1)
    }
}

static set: &'static[f32] = &[3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0];

fn main() {
    for (a, b) in set.iter().enumerate(){
        println!("value {}, number {}", b, a);
    }
}
