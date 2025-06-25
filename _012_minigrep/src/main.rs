use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = _012_minigrep::Config::load()?;
    return _012_minigrep::run(config);
}
