//For prolétariat users
fn main() {
	println!("cargo:rustc-link-search=ppmaio/");
}
//For MacOS user
/*use cc;

fn main() {
	cc::Build::new().file("ppmaio/ppma_io.c").compile("foo");
}*/
