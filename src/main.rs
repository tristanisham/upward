use wallpaper;

fn main() {
    // Sets the wallpaper for the current desktop from a URL.
    wallpaper::set_from_url("https://source.unsplash.com/random").unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}

// Thank you https://crates.io/crates/wallpaper#README. Christopher Knight, if that's you. You're a saint.