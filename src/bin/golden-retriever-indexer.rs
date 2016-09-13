extern crate csearch;

use csearch::indexer::app;

fn main() {
    let matches = app::make().get_matches();
    if let Some(cmd) = matches.subcommand_matches("ls") {
        println!("Command: {:?}", cmd);
    } else {
        println!("No command given.");
    }
}
