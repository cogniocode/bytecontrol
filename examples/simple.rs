use bytecontrol::composition::compose_rules;
use bytecontrol::condition::condition;
use bytecontrol::rule::Rule;
use bytecontrol::rules::string::length;

struct User {
    pub username: String
}

fn main() {
    let user = User {
        username: String::from("username")
    };

    let rule = compose_rules()
        .rule(length(10, 20))
        .rule(condition(|val: &String| val.starts_with("user"), String::from("invalid name")))
        .compose();

    match rule.apply(&user.username) {
        Ok(_) => println!("All good."),
        Err(err) => println!("{}", err)
    };
}