use std::env;


static SET: &'static[f32] = &[3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0];
const NEG_TAO: f32 = -6.2832;
const TAO: f32 = 6.2832;


fn main() {
    //gets arguments from cli
    let args: Vec<String> = env::args().collect();
    let arg: f32 = args[1].parse().unwrap();
    let first = arg.to_string().chars().nth(0).unwrap(); 
    let c = constrain(arg, first);
    //isolates for the first character
    let mut output = c;
    //main loop to calculate the taylor series
    for (count, set_value) in SET.iter().enumerate(){
        if (count + 1) % 2 == 0{
            output += c.powf(*set_value) / factorial(*set_value);
        }
        else{
            output -= c.powf(*set_value) / factorial(*set_value);
        }    
    }

    println!("{:.5}", output);
}

//puts the value between -2PI and 2PI 
//
//num - the number to be reduced
//first it the first character of the number
fn constrain(num: f32, first: char) -> f32{
    let mut reduced = num;
    if first == '-'{
        while reduced < NEG_TAO{
            reduced += TAO;
        }
    }
    else{
        while reduced > TAO{
            reduced -= TAO;
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
