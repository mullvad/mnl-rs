extern crate pkg_config;

#[cfg(feature = "mnl-1-0-4")]
const MIN_VERSION: &str = "1.0.4";
#[cfg(not(feature = "mnl-1-0-4"))]
const MIN_VERSION: &str = "1.0.3";


#[cfg(target_os = "linux")]
fn main() {
    println!("Minimum libmnl version: {}", MIN_VERSION);
    pkg_config::Config::new()
        .atleast_version(MIN_VERSION)
        .probe("libmnl")
        .unwrap();
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This crate does nothing on non-Linux");
}
