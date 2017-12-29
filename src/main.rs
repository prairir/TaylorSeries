
static set: &'static[f32] = &[3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0];
const negTao: f32 = -6.28;
const Tao: f32 = 6.28;


fn main() {
    let c = 9.42;
    let first = c.to_string().chars().nth(0).unwrap(); 
    //for (a, b) in set.iter().enumerate(){
    //    println!("value {}, number {}", b, a);
    //}
    println!("{}", constrain(c, first));
}

//puts the value between -2PI and 2PI 
//
//num - the number to be reduced
//first it the first character of the number
fn constrain(num: f32, first: char) -> f32{
    let mut reduced = num;
    if first == '-'{
        while reduced < negTao{
            reduced += Tao;
            println!("{}", reduced);
        }
    }
    else{
        while reduced > Tao{
            reduced -= Tao;
            println!("{}", reduced)
        }
    }
    return reduced;
}

//calculates the factorial of a number
//
//num - the number to be factorialized
fn factorial(num: f32) -> f32{
    if num==0.0{
        return 1.0;
    }
    else{
        return num * factorial(num -1.0)
    }
}
