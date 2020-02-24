use clap::{crate_version, App, AppSettings, SubCommand};
use webbrowser;

fn main() {
    let matches = App::new("shunkakinoki")
        .version(crate_version!())
        .author("Shun Kakinoki <shunkakinoki@gmail.com>")
        .subcommand(SubCommand::with_name("blog").about("Opens blog.shunkakinoki.com"))
        .subcommand(SubCommand::with_name("home").about("Opens shunkakinoki.com"))
        .subcommand(SubCommand::with_name("journal").about("Opens journal.shunkakinoki.com"))
        .subcommand(
            App::new("github")
                .about("Opens github.com")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("shunkakinoki").about("Opens shunkakinoki/shunkakinoki"),
                )
                .subcommand(SubCommand::with_name("journal").about("Opens shunkakinoki/journal"))
                .subcommand(SubCommand::with_name("website").about("Opens shunkakinoki/website"))
                .subcommand(SubCommand::with_name("notebook").about("Opens shunkakinoki/notebook"))
                .subcommand(SubCommand::with_name("pitch").about("Opens shunkakinoki/pitch"))
                .subcommand(SubCommand::with_name("dotfiles").about("Opens shunkakinoki/dotfiles")),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("blog", Some(_)) => if webbrowser::open("https://blog.shunkakinoki.com").is_ok() {},
        ("home", Some(_)) => if webbrowser::open("https://home.shunkakinoki.com").is_ok() {},
        ("journal", Some(_)) => if webbrowser::open("https://journal.shunkakinoki.com").is_ok() {},
        ("github", Some(github_matches)) => match github_matches.subcommand() {
            ("shunkakinoki", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/shunkakinoki").is_ok() {};
            }
            ("journal", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/journal").is_ok() {};
            }
            ("website", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/website").is_ok() {};
            }
            ("notebook", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/notebook").is_ok() {};
            }
            ("pitch", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/pitch").is_ok() {};
            }
            ("dotfiles", Some(_)) => {
                if webbrowser::open("https://github.com/shunkakinoki/dotfiles").is_ok() {};
            }
            _ => if webbrowser::open("https://github.com").is_ok() {},
        },
        _ => unreachable!(),
    };
}
