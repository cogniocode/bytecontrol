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

    todo!();
}