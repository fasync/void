fn main() {
    // We need that to build under FreeBSD
    println!(r"cargo:rustc-link-search=/usr/local/lib/");
}
