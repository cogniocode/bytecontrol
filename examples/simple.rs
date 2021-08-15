use bytecontrol::composition::compose_rules;
use bytecontrol::rule::string::length;
use bytecontrol::rule::Rule;

struct User {
    pub username: String
}

fn main() {
    let user = User {
        username: String::from("cognio")
    };

    let rule = compose_rules()
        .rule(length(10, 20))
        .rule(|val: &String| val.starts_with("cog"))
        .compose();

    match rule.apply(&user.username) {
        Ok(_) => println!("All good."),
        Err(err) => println!("{}", err)
    };
}