use rocket::http::CookieJar;
#[macro_use]
extern crate rocket;

#[get("/?<name>")]
fn index(name: Option<&str>, cookies: &CookieJar<'_>) -> String {
    let intro: String = match name {
        None => "Message: ".to_string(),
        Some(name) => match name {
            "" => "Message: ".to_string(),
            name => format!("Message from {}: ", name),
        },
    };

    match cookies.get("message") {
        None => format!("No message!"),
        Some(message_cookie) => format!("{}{}", intro, message_cookie.value()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
