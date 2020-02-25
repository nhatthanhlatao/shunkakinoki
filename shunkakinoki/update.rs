use self_update;

pub fn update() -> Result<(), Box<dyn ::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("shunkakinoki")
        .repo_name("shunkakinoki")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);

    let asset = releases[0].asset_for(&self_update::get_target()).unwrap();
    let tmp_dir = self_update::TempDir::new_in(::std::env::current_dir()?, "self_update")?;
    let tmp_tarball_path = tmp_dir.path().join(&asset.name);
    let tmp_tarball = ::std::fs::File::open(&tmp_tarball_path)?;
    self_update::Download::from_url(&asset.download_url).download_to(&tmp_tarball)?;
    let bin_name = std::path::PathBuf::from("self_update_bin");
    bin_name.to_str();
    Ok(())
}

pub fn list() -> Result<(), Box<dyn ::std::error::Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("shunkakinoki")
        .repo_name("shunkakinoki")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases[0]);
    Ok(())
}
