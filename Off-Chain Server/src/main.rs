// main.rs

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "StreamPay Off-Chain Server is running!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
