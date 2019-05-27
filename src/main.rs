// What does that mean?
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/?<query_string>")]
fn string_adder(query_string: String) -> String {
    let split_string: Vec<&str> = query_string.split("+").collect();

    let mut response_string = String::from("The sum of ");

    for x in 0..split_string.len() {
        response_string.push_str(split_string[x]);

        if x != split_string.len()-1 {
            response_string.push_str(", ");
        }
    }

    let sum: i32 = adder(query_string);
    response_string.push_str(&format!(" is {}.", &sum.to_string()));

    return response_string;
}

fn adder(query_string: String) -> i32 {
    let split_string = query_string.split("+");

    let mut sum: i32 = 0;
    for x in split_string {
        let s: i32 = x.parse().unwrap();
        sum += s;
    }

    return sum;
}

fn main() {
    rocket::ignite()
        .mount("/", rocket_contrib::serve::StaticFiles::from("static"))
        .mount("/adder", routes![string_adder])
        .launch();
}
