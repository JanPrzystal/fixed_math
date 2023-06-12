mod mixed_number;

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
    let fixed1 = Number::from_float(3.1);
    let fixed2 = Number::from_float(-13.641);
    let fixed3 = Number::from_decimal(118187, 3);

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

        let decimal = Number::from_float(fl);

        println!("as fixed point {}", decimal.fixed);

        println!("as int {}", decimal.to_int());

        println!("as float {}", decimal.to_float());

        println!("as string {}", decimal);

        s=get_input("input decimal: ");

    }
}

fn main() {
    demo();

    poll_number();
}

