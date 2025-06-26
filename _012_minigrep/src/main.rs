use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _config = _012_minigrep::Config::load2();
    let config = _012_minigrep::Config::load(env::args())?;
    return _012_minigrep::run(config);
}
