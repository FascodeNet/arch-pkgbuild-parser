extern crate clap;
use clap::{App, Arg, SubCommand};
fn main() {
    let app = App::new("arch-pkgbuild-parser")
    .version("0.1.0")
    .author("kokkiemouse <Twitter -> @kokkiemouse>")
    .about("PKGBUILD PARSER")
    .arg(Arg::with_name("json")
    .help("output json")
    .short("j")
    .long("json"));
    let matches = app.get_matches();
    println!("tdn");
}
