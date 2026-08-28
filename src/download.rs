use std::error::Error;
use std::fs::File;
use std::io;

/// Downloads `url` into the user's cache directory and returns the path it was
/// written to.
///
/// This mirrors what `wallpaper`'s `from_url` feature does internally. We keep
/// our own copy so the crate can be depended on without that feature, which
/// would otherwise drag in the unmaintained reqwest 0.11 / hyper 0.14 / h2 0.3
/// stack.
pub fn image(url: &str) -> Result<String, Box<dyn Error>> {
    let file_path = dirs::cache_dir().ok_or("no cache dir")?.join("wallpaper");

    let mut file = File::create(&file_path)?;
    let mut body = ureq::get(url).call()?.into_body();
    io::copy(&mut body.as_reader(), &mut file)?;

    Ok(file_path.to_str().ok_or("no file path")?.into())
}
