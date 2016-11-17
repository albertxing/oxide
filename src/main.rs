extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;
use std::env;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
	let port = env::args().nth(1).unwrap_or("8080".to_string()).parse::<u16>().unwrap();
	let mut mount = Mount::new();
	mount.mount("/", Static::new(Path::new("./")));
	println!("listening on port {:?}", port);
	Iron::new(mount).http(("127.0.0.1", port)).unwrap();
}
