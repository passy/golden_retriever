extern crate csearch;
extern crate clap;

use csearch::indexer::app;
use clap::ErrorKind;
use std::env;

#[test]
fn test_matches_version() {
    let res = app::make().get_matches_from_safe(vec!["golden-retriever-indexer", "--version"]);

    let err = res.unwrap_err();

    assert_eq!(err.message, "");
    assert_eq!(err.kind, ErrorKind::VersionDisplayed);
}

#[test]
fn test_matches_ls() {
    let res = app::make()
        .get_matches_from_safe(vec!["golden-retriever-indexer", "ls"])
        .expect("should be an Ok result");

    let subcommand = res.subcommand_matches("ls").expect("ls subcommand should exist");
    assert!(subcommand.args.is_empty(), "shouldn't have any arguments")
}

#[test]
fn test_matches_add() {
    let res = app::make()
        .get_matches_from_safe(vec!["golden-retriever-indexer", "add"])
        .expect("should be an Ok result");

    let subcommand = res.subcommand_matches("add").expect("add subcommand should exist");
    assert!(subcommand.args.is_empty(), "shouldn't have any arguments")
}

#[test]
fn test_matches_rm() {
    let res = app::make()
        .get_matches_from_safe(vec!["golden-retriever-indexer", "rm"])
        .expect("should be an Ok result");

    let subcommand = res.subcommand_matches("rm").expect("rm subcommand should exist");
    assert!(subcommand.args.is_empty(), "shouldn't have any arguments")
}

#[test]
fn test_matches_status() {
    let res = app::make()
        .get_matches_from_safe(vec!["golden-retriever-indexer", "status"])
        .expect("should be an Ok result");

    let subcommand = res.subcommand_matches("status").expect("status subcommand should exist");
    assert!(subcommand.args.is_empty(), "shouldn't have any arguments")
}

#[test]
fn test_matches_run() {
    let res = app::make()
        .get_matches_from_safe(vec!["golden-retriever-indexer", "run"])
        .expect("should be an Ok result");

    let subcommand = res.subcommand_matches("run").expect("run subcommand should exist");
    assert!(subcommand.args.is_empty(), "shouldn't have any arguments")
}
