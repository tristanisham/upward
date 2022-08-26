use clap::Parser;
use wallpaper;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// x dimensions for new dekstop background
    #[clap(short, long, value_parser, default_value_t = 1920)]
    x: i32,

    /// y dimensions for new dekstop background
    #[clap(short, long, value_parser, default_value_t = 1080)]
    y: i32,

    #[clap(short, long, value_parser, default_value_t = String::from(""))]
    search: String,
}

fn main() {
    // if std::env::consts::OS != "windows" {
    //     println!("Warning! This program has only been tested on Windows and may not perform as expected.\nCheck https://github.com/tristanisham/upward for compatability updates");
    // }

    let args = Args::parse();
    let screen_size: (i32, i32) = (args.x, args.y);
    let queries: Vec<&str> = args.search.trim().split_ascii_whitespace().collect();
    let search_terms = queries.join(",");
    // Sets the wallpaper for the current desktop from a URL.
    let url = format!(
        "https://source.unsplash.com/random/{}x{}?{}",
        screen_size.0, screen_size.1, search_terms
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

    println!("Wallpaper updated! {:?}", wallpaper::get());
}

// Thank you https://crates.io/crates/wallpaper#README. Christopher Knight, if that's you. You're a saint.
