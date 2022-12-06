use wallpaper;

use crate::cli::Args;
mod cli;

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[clap(author, about, version, long_about = None)]
// struct Args {
//     /// x dimensions for new dekstop background
//     #[clap(short, long, value_parser, default_value_t = 1920)]
//     x: i32,

//     /// y dimensions for new dekstop background
//     #[clap(short, long, value_parser, default_value_t = 1080)]
//     y: i32,

//     #[clap(short, long, value_parser, default_value_t = String::from(""))]
//     search: String,

// }

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();

    let args = Args::parse(&args);
    // Sets the wallpaper for the current desktop from a URL.
    let url = format!(
        "https://source.unsplash.com/random/{}x{}?{}",
        args.x, args.y, args.description.join(",")
    );
    // println!("{url}");
    wallpaper::set_from_url(&url).unwrap();
    // Returns the wallpaper of the current desktop.
    // println!(
    //     "x:{}, y:{} | {:?}",
    //     screen_size.0,
    //     screen_size.1,
    //     wallpaper::get()
    // );

    println!("Wallpaper updated! {}", wallpaper::get().unwrap_or(url));
}

// Docs & Thanks
// Thank you https://crates.io/crates/wallpaper#README. Christopher Knight, if that's you. You're a saint.
// https://awik.io/generate-random-images-unsplash-without-using-api/