use std::io;

struct RomanVal<'a> {
    val: i32,
    roman: &'a str,
}

fn main() {
    println!("Enter a number");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let num: i32 = input.parse().unwrap();

    println!("You entered '{}'", num);

    let roman = to_roman(num);
    println!("That's '{}' in Roman Numerals", roman);
}

fn to_roman(n: i32) -> String {
    if n > 3999 || n <= 0 {
        // better to return a Result enum?
        panic!("number must be between 1 and 3999, cus I don't want to code stuff like V̅ etc")
    }

    let rvs = vec![
        RomanVal { val: 1000, roman: "M" },
        RomanVal { val: 900, roman: "CM" },
        RomanVal { val: 500, roman: "D" },
        RomanVal { val: 400, roman: "CD" },
        RomanVal { val: 100, roman: "C" },
        RomanVal { val: 90, roman: "XC" },
        RomanVal { val: 50, roman: "L" },
        RomanVal { val: 40, roman: "XL" },
        RomanVal { val: 10, roman: "X" },
        RomanVal { val: 9, roman: "IX" },
        RomanVal { val: 5, roman: "V" },
        RomanVal { val: 4, roman: "IV" },
        RomanVal { val: 1, roman: "I" },
    ];

    let mut result: String = String::new();
    let mut n = n;
    while n > 0 {
        for rv in &rvs {
            if n >= rv.val {
                n -= rv.val;

                // this seems like a hack
                result = format!("{}{}", result, rv.roman);

                // it'd be better to "i--" here with a traditional for loop
                // so we don't have to start over from "M"
                break;
            }
        }
    }

    result
}
