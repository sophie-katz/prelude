use openapi::models::user_response::{UserResponse};

fn main() {
    let x = UserResponse::new();

    println!("x: {:#?}", x);
}
