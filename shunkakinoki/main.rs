use clap::{crate_version, App};
use webbrowser;

fn main() {
    let matches = App::new("shunkakinoki")
        .version(crate_version!())
        .author("Shun Kakinoki <shunkakinoki@gmail.com>")
        .subcommand(App::new("blog").about("Opens blog.shunkakinoki.com"))
        .subcommand(App::new("home").about("Opens shunkakinoki.com"))
        .subcommand(App::new("journal").about("Opens journal.shunkakinoki.com"))
        .get_matches();

    if matches.is_present("blog") {
        if webbrowser::open("https://blog.shunkakinoki.com").is_ok() {};
    }

    if matches.is_present("home") {
        if webbrowser::open("https://shunkakinoki.com").is_ok() {};
    }

    if matches.is_present("journal") {
        if webbrowser::open("https://journal.shunkakinoki.com").is_ok() {};
    }
}
