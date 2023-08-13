#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        title: "Rocket Intro"
    })
}

#[get("/about")]
fn about() -> &'static str {
    "About me"
}

#[get("/contact")]
fn contact() -> &'static str {
    "Contact me"
}

#[get("/")]
fn profile() -> &'static str {
    "get profile"
}

#[post("/")]
fn create_profile() -> &'static str {
    "create profile"
}

#[put("/")]
fn update_profile() -> &'static str {
    "update profile"
}

#[delete("/")]
fn delete_profile() -> &'static str {
    "Delete profile"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about, contact])
    .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
    .attach(Template::fairing())
}
