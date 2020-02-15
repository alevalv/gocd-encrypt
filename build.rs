// build.rs
extern crate clap;

use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = get_arguments();
    app.gen_completions(
        "gocd-encrypt",           // We specify the bin name manually
        Shell::Bash,      // Which shell to build completions for
        &outdir); // Where write the completions to
    app.gen_completions(
        "gocd-encrypt",           // We specify the bin name manually
        Shell::Zsh,      // Which shell to build completions for
        &outdir); // Where write the completions to
}
