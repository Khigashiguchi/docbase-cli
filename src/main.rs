#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate serde_json;

mod docbase;

use docopt::Docopt;
use docbase::Docbase;

const USAGE: &'static str = "
to use DocBase cli
USAGE:
    docbase-cli
    docbase-cli post
    docbase-cli (-h | --help)
    docbase-cli --version

Options:
    -h, --help      Show this screen.
    --version       Show version.
";

#[derive(Debug, Deserialize)]
pub struct Args {
    cmd_post: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

    let mut docbase = Docbase::new();
    docbase.run(args);
    //println!("{:?}", args);
}
