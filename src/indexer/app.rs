use clap::{App, SubCommand};

pub fn make() -> App<'static, 'static> {
    App::new("golden-retriever-indexer")
        .version(crate_version!())
        .about("Golden Retriever Indexer")
        .subcommand(SubCommand::with_name("ls").about("list indexed directories"))
        .subcommand(SubCommand::with_name("add").about("add a new directory to be indexed"))
        .subcommand(SubCommand::with_name("rm").about("remove a directory from the index"))
        .subcommand(SubCommand::with_name("status").about("show the current status of the indexer"))
        .subcommand(SubCommand::with_name("run").about("run the indexer"))
}
