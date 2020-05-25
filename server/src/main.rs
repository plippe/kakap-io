#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;

fn static_files_folder() -> String {
    match Environment::active() {
        Ok(Environment::Production) => "public/",
        _ => "../client/dist/",
    }
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from(static_files_folder()))
        .launch();
}
