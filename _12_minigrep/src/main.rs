use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _config = _12_minigrep::Config::load2();
    let config = _12_minigrep::Config::load(env::args())?;
    return _12_minigrep::run(config);
}
