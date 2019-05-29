// What does that mean?
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


fn rpn(s: String) -> String {

    let mut command_stack: Vec<&str> = s.split_whitespace().rev().collect();
    let mut calculator_stack: Vec<f64> = vec![];

    while command_stack.len() > 0 {
        // Take from command string and place on the stack
        let input: &str = command_stack.pop().unwrap();

        // If the input is a number
        if input.parse::<f64>().is_ok() {
            // Add to calculator stack
            calculator_stack.push(input.parse::<f64>().unwrap());
        } else {
            // Do what the operator says
            let b = calculator_stack.pop().unwrap();
            let a = calculator_stack.pop().unwrap();

            if input == "+" {
                calculator_stack.push(a+b);
            }

            if input == "*" {
                calculator_stack.push(a*b);
            }

            if input == "-" {
                calculator_stack.push(a-b);
            }

            if input == "/" {
                calculator_stack.push(a/b);
            }
        }
    }

    // Computation is done, the answer should be left
    return calculator_stack[0].to_string();
}


#[get("/?<query_string>")]
fn string_rpn(query_string: String) -> String {
    return rpn(query_string);
}

fn main() {
    rocket::ignite()
        .mount("/", rocket_contrib::serve::StaticFiles::from("static"))
        .mount("/rpn", routes![string_rpn])
        .launch();
}
