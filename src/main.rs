#[macro_use]
extern crate serde_derive;
extern crate docopt;


use docopt::Docopt;

static SET: &'static[f32] = &[3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0];
const NEG_TAO: f32 = -6.2832;
const TAO: f32 = 6.2832;


const USAGE: &'static str = "
taylor
Generates an estimate sin using a taylor series

Usage:
    taylor <number>
    taylor (-h | --help)

Options:
    -h, --help      Shows this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
   arg_number: f32,
}

fn main() {
    //gets arguments from cli
    let args: Args = Docopt::new(USAGE).
        and_then(|d| d.deserialize()).
        unwrap_or_else(|e| e.exit());
    let arg = args.arg_number;
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
