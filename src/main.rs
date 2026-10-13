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
    let path = download_image(&url).unwrap();
    wallpaper::set_from_path(&path).unwrap();
    // Returns the wallpaper of the current desktop.
    // println!(
    //     "x:{}, y:{} | {:?}",
    //     screen_size.0,
    //     screen_size.1,
    //     wallpaper::get()
    // );

    println!("Wallpaper updated! {}", wallpaper::get().unwrap_or(url));
}

/// Downloads `url` into the user's cache directory and returns the file path.
/// Mirrors what `wallpaper::set_from_url` does internally.
fn download_image(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir")?;
    let file_path = cache_dir.join("wallpaper");

    let mut response = ureq::get(url).call()?;
    let mut file = std::fs::File::create(&file_path)?;
    std::io::copy(&mut response.body_mut().as_reader(), &mut file)?;

    Ok(file_path.to_str().ok_or("no file path")?.into())
}

// Docs & Thanks
// Thank you https://crates.io/crates/wallpaper#README. Christopher Knight, if that's you. You're a saint.
// https://awik.io/generate-random-images-unsplash-without-using-api/