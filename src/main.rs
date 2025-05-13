mod mixed_number;
use std::time::{Duration, Instant};
use rand::Rng;

include!("mixed_number.rs");

pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn demo() {
    //create a few numbers, perform math operations and print results
    let fixed1 = num32::from_float(3.1);
    let fixed2 = num32::from_float(-13.641);
    let fixed3 = num32::from_decimal(118187, 3);

    let fixed12 = fixed1 * fixed2;
    let fixed32 = fixed3 + fixed2;
    let fixed31 = fixed3 / fixed1;

    println!("number 1: {}", fixed1);
    println!("number 2: {}", fixed2);
    println!("number 3: {}", fixed3);

    println!("n1 * n2 = {} as int", fixed12.to_int());
    println!("n2 + n3 =  {} as int", fixed32.to_int());
    println!("n3 / n1 = {} as int", fixed31.to_int());

    println!("{} + {} = {}", fixed3, fixed2, fixed32.to_float());
    println!("{} / {} = {}", fixed3, fixed1, fixed31.to_float());
}

fn poll_number(){
    let mut s: String;
    s=get_input("input decimal: ");
    while s!="e" {
        let fl: f32 = s.parse::<f32>().unwrap();

        println!("your number is {}", fl);

        let decimal = num32::from_float(fl);

        println!("as fixed point {}", decimal.fixed);

        println!("as int {}", decimal.to_int());

        println!("as float {}", decimal.to_float());

        println!("as string {}", decimal);

        s=get_input("input decimal: ");

    }
}

fn measure_fixed(float_array: &Vec<f32>) {
    let fixed_array: Vec<num32> = float_array.iter()
        .map(|x| num32::from_float(*x))
        .collect();

    let now = Instant::now();

    let total: num32 = fixed_array.iter().sum();

    let elapsed = now.elapsed().as_micros();
    println!("fixed sum result {}, in {}us", total.to_float(), elapsed);
}

fn measure_float(float_array: &Vec<f32>) {

    let now = Instant::now();

    let total: f32 = float_array.iter().sum();

    let elapsed = now.elapsed().as_micros();
    println!("float sum result {}, in {}us", total, elapsed);
}

fn main() {
    demo();

    let mut rng = rand::thread_rng();

    let float_array: Vec<f32> = (0..100000)
        .map(|_| rng.gen_range(-10.0..=10.0))
        .collect();

    
    measure_fixed(&float_array);
    measure_float(&float_array);
    // poll_number();
}

