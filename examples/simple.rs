use bytecontrol::rule::Rule;
use bytecontrol::rules::string::length;

fn main() {
    let first_name = "Mike";

    let rule = length(2, 32);

    match rule.apply(&String::from(first_name)) {
        Ok(_) => println!("All good."),
        Err(err) => println!("Errors: {}", err)
    };
}