#![feature(plugin)]
#![feature(decl_macro)]

use std::path::{Path, PathBuf};

use rocket::{
    fairing::AdHoc,
    response::{NamedFile, Redirect},
    Request, State,
};
use rocket_codegen::{catch, catchers, get, routes, uri};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Btn {
    label:    String,
    link:     String,
    new_page: bool,
}

impl Btn {
    pub fn new<L: ToString, I: ToString>(label: L, link: I, new_page: bool) -> Self {
        Btn {
            label: label.to_string(),
            link: link.to_string(),
            new_page,
        }
    }
}

#[derive(Serialize)]
struct BaseTemplateContext {
    title:     String,
    subheader: String,
    buttons:   Vec<Btn>,
}

impl BaseTemplateContext {
    pub fn new<T: ToString, S: ToString>(title: T, subheader: S, buttons: Vec<Btn>) -> Self {
        BaseTemplateContext {
            title: title.to_string(),
            subheader: subheader.to_string(),
            buttons,
        }
    }
}

#[get("/")]
fn index() -> Template {
    let github = Btn::new("GitHub", "https://github.com/L3afMe/", true);
    let projects = Btn::new("Projects", "/projects", false);
    let ctx = BaseTemplateContext::new("l3af.me", "fuck knows anymore", vec![github, projects]);
    Template::render("base", ctx)
}

#[get("/projects")]
fn projects() -> Template {
    let inori = Btn::new("Inori-rs", "/inori-rs", false);
    let miyuki = Btn::new("Miyuki-rs", "/miyuki-rs", false);
    let ctx = BaseTemplateContext::new("Projects", "a few mediocre projects", vec![inori, miyuki]);
    Template::render("base", ctx)
}

#[get("/inori-rs")]
fn inori_rs() -> Template {
    let github = Btn::new("GitHub", "https://github.com/L3afMe/inori-rs", true);
    let ctx = BaseTemplateContext::new("Inori-rs", "a pretty shitty selfbot", vec![github]);
    Template::render("base", ctx)
}

#[get("/miyuki-rs")]
fn miyuki_rs() -> Template {
    let ctx = BaseTemplateContext::new("Miyuki-rs", "coming soon™", Vec::new());
    Template::render("base", ctx)
}

#[catch(404)]
fn not_found(_req: &Request) -> Redirect {
    Redirect::to(uri!(index))
}

struct AssetsDir(String);

#[get("/assets/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, projects, inori_rs, miyuki_rs, assets])
        .register(catchers![not_found])
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Assets Config", |rocket| {
            let assets_dir = rocket.config().get_str("assets_dir").unwrap_or("assets/").to_string();

            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))
        .launch();
}
