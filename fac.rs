use std::io::{self, BufRead, Lines, StdinLock};

const LABELS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

macro_rules! next_line {
    ($it:expr) => { $it.next().unwrap().unwrap(); };
}

macro_rules! copy_string {
    ($from:expr, $to:expr) => {
        $to.clear();

        {
            let mut trim = true;

            for c in $from.chars() {
                if trim && c == '0' {
                    if $from.len() == 1 {
                        $to.push(c);
                        break;
                    } else {
                        continue;
                    }
                }

                trim = false;
                $to.push(c);
            }
        }
    };
}

macro_rules! insert_at {
    ($s:expr, $i:expr, $v:expr, $res:expr) => {
        for (j, c) in $s.chars().enumerate() {
            if $i != j {
                $res.push(c);
            } else {
                $res.push($v);
            }
        }
    };
}

fn minus_one_digit_aux(b: &String, i: usize, result: &mut String) {
    let minus =
        result.chars().nth(i).unwrap() as i8 -
        b.chars().nth(0).unwrap() as i8;

    let mut aux = String::with_capacity(result.len());

    if minus < 0 {
        insert_at!(result, i, LABELS[(10 + minus) as usize], aux);
        copy_string!(aux, result);

        return minus_one_digit_aux(&minus.abs().to_string(), i - 1, result);
    } else {
        insert_at!(result, i, LABELS[minus.abs() as usize], aux);
        copy_string!(aux, result);
    }
}

fn minus_one_digit(a: &String, b: &String) -> String {
    let mut result = String::with_capacity(a.len());

    copy_string!(a, result);
    minus_one_digit_aux(b, a.len() - 1, &mut result);

    result
}

fn plus_aux(a: &String, b: &String) -> String {
    let mut result = String::with_capacity(a.len() + 2);

    let b_chars = b.chars().rev();
    let mut carry: u8 = 0;

    let mut i = 0;

    {
        let a_chars = a.chars().rev();

        for (l, r) in a_chars.zip(b_chars) {
            let sum =
                l.to_digit(10).unwrap() +
                r.to_digit(10).unwrap() +
                carry as u32;

            if sum > 9 {
                carry = 1;
                result.push(LABELS[(sum - 10) as usize]);
            } else {
                carry = 0;
                result.push(LABELS[sum as usize]);
            }

            i += 1;
        }
    }

    {
        let a_chars = &(a.chars().rev().collect::<String>())[i..];

        for c in a_chars.chars() {
            let sum = c.to_digit(10).unwrap() + carry as u32;

            if sum > 9 {
                carry = 1;
                result.push('0');
            } else {
                carry = 0;
                result.push(LABELS[sum as usize]);
            }

            i += 1;
        }
    }

    if carry > 0 { result.push('1'); }

    result.chars().rev().collect()
}

fn plus(a: &String, b: &String) -> String {
    if a.len() > b.len() { plus_aux(a, b) } else { plus_aux(b, a) }
}

fn multiply_aux(a: &String, b: &String, i: u8) -> String {
    let mut result = String::with_capacity(b.len());

    if i < 2 {
        copy_string!(b, result);
    } else {
        copy_string!(multiply_aux(a, &plus(a, b), i - 1), result);
    }

    result
}

fn multiply(a: u8, b: &String) -> String {
    multiply_aux(b, b, a)
}

fn fact(n: String) -> String {
    if n == "0".to_string() || n == "".to_string() { return "1".to_string(); }

    let fact_value = fact(minus_one_digit(&n, &"1".to_string()));

    multiply(n.parse::<u8>().unwrap(), &fact_value)
}

fn test_case(it: &mut Lines<StdinLock<'_>>) -> String {
    let n: String = next_line!(*it);

    fact(n)
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let n = next_line!(iter).parse::<u8>().unwrap();
    let mut i = 0;

    while i < n {
        i += 1;
        println!("{}", test_case(&mut iter));
    }
}