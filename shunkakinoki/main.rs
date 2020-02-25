use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use webbrowser;

mod update;

fn main() {
    let matches = App::new("shunkakinoki")
        .version(crate_version!())
        .author("Shun Kakinoki <shunkakinoki@gmail.com>")
        .subcommand(SubCommand::with_name("blog").about("Opens blog.shunkakinoki.com"))
        .subcommand(SubCommand::with_name("home").about("Opens shunkakinoki.com"))
        .subcommand(SubCommand::with_name("journal").about("Opens journal.shunkakinoki.com"))
        .subcommand(SubCommand::with_name("pitch").about("Opens pitch.shunkakinoki.com"))
        .subcommand(SubCommand::with_name("notebook").about("Opens notebook.shunkakinoki.com"))
        .subcommand(SubCommand::with_name("update").about("Update shunkakinoki cli"))
        .subcommand(
            App::new("github")
                .about("Opens github.com")
                .subcommand(
                    App::new("shunkakinoki")
                        .about("Opens shunkakinoki/shunkakinoki")
                        .arg(
                            Arg::with_name("pull-request")
                                .long("pr")
                                .short("p")
                                .required(false)
                                .help("Go to PR"),
                        )
                        .setting(AppSettings::ArgRequiredElseHelp),
                )
                .subcommand(App::new("journal").about("Opens shunkakinoki/journal"))
                .subcommand(App::new("website").about("Opens shunkakinoki/website"))
                .subcommand(App::new("notebook").about("Opens shunkakinoki/notebook"))
                .subcommand(App::new("pitch").about("Opens shunkakinoki/pitch"))
                .subcommand(App::new("dotfiles").about("Opens shunkakinoki/dotfiles")),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match matches.subcommand() {
        ("blog", Some(_)) => if webbrowser::open("https://blog.shunkakinoki.com").is_ok() {},
        ("home", Some(_)) => if webbrowser::open("https://home.shunkakinoki.com").is_ok() {},
        ("journal", Some(_)) => if webbrowser::open("https://journal.shunkakinoki.com").is_ok() {},
        ("pitch", Some(_)) => if webbrowser::open("https://pitch.shunkakinoki.com").is_ok() {},
        ("notebook", Some(_)) => {
            if webbrowser::open("https://notebook.shunkakinoki.com").is_ok() {}
        }
        // ("update", Some(_)) => update::update(),
        ("github", Some(github_matches)) => match github_matches.subcommand() {
            ("shunkakinoki", Some(shunkakinoki_matches)) => {
                if shunkakinoki_matches.is_present("pull-request")
                    && webbrowser::open("https://github.com/shunkakinoki/shunkakinoki/pull").is_ok()
                {
                }
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
