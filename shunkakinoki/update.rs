use self_update;

#[allow(dead_code)]
pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("shunkakinoki")
        .repo_name("shunkakinoki")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);
    Ok(())
}
