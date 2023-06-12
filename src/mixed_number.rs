use std::string::String;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

const PRECISION: i32 = 8;
const PRECISION_MASK: i32 = (1 << PRECISION) - 1;

fn get_largest_decimal(mut number: i32) -> i32 {//shifts the number to the leftmost available position in the decimal system
    let mut check = Some(number);//used to check overflow
    while check.is_some() {
        number = check.unwrap();
        check = number.checked_mul(10);
    }
    number
}

struct Number{
    fixed: i32 //fixed point value
}

impl Number{
    fn from_float (decimal: f32) -> Self {
        let fixed_value = (decimal * (1 << PRECISION) as f32).round() as i32;
        Number{fixed: fixed_value}
    }
    
    fn from_decimal (number: i32, digits_after_decimal: u32) -> Self {
        //example - 123.456 would be number = 123456 and digits_after_decimal 3
        //TODO check the size of digits after decimal
        let decimal_multiplier: i32 = (10 as i32).pow(digits_after_decimal);
        let whole: i32 = number / decimal_multiplier << PRECISION;
        let fraction: i32 = number % decimal_multiplier * PRECISION_MASK / decimal_multiplier;
        Number{fixed: whole + fraction}
    }

    fn split(&self) -> (i32, i32){//return whole and fractional numbers
        let mut whole = self.fixed >> PRECISION;
        let mut fraction = self.fixed & PRECISION_MASK;
        if whole < 0 && fraction != 0{
            fraction = fraction ^ PRECISION_MASK;
            whole += 1;
        }
        (whole, fraction)
    }

    fn to_int(&self) -> i32 {
        let frac = self.fixed & (1 << (PRECISION-1));
        let add = if frac==0 {0} else {1};
        (self.fixed >> PRECISION) + add
    }

    fn to_float(&self) -> f32 {
        let (whole, fraction) = self.split();
        let f_value: f32 = whole as f32 + ((fraction as f32 / PRECISION_MASK as f32) * if whole>=0 {1.0} else {-1.0});
        f_value
    }

    fn to_string(&self) -> String {//returns string with the number in decimal system
        let mut ret = String::from("");
        let (whole, fraction) = self.split();

        //divide the fraction integer by its highest possible number to get the fraction number
        //the fraction is shifted to the leftmost position in the decimal system to get the highest precision without converting to float
        let mut f_value: i32 = fraction;
        if f_value != 0 {
            f_value = get_largest_decimal(f_value);
            f_value = f_value / PRECISION_MASK;
        }

        ret.push_str(&whole.to_string());
        ret.push('.');
        ret.push_str(&f_value.to_string());

        ret
    }
}

impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {fixed: self.fixed + other.fixed}
    }
}

impl Mul for Number {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {fixed: (self.fixed * other.fixed) >> PRECISION}
    }
}

impl Sub for Number {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {fixed: self.fixed - other.fixed}
    }
}

impl Div for Number {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let whole = (self.fixed / other.fixed) << PRECISION;
        // let fraction = self.fixed % other.fixed;
        Self {fixed: whole}
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Copy for Number { }

impl Clone for Number {
    fn clone(&self) -> Number {
        *self
    }
}
