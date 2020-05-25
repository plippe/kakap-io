#![feature(decl_macro, option_result_contains, proc_macro_hygiene)]

#[macro_use]
extern crate serde_derive;

mod chainable;
mod error;
mod platforms;

use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;

fn static_files_folder() -> String {
    (match Environment::active() {
        Ok(Environment::Production) => "public/",
        _ => "../client/dist/",
    }).to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from(static_files_folder()))
        .launch();
}
