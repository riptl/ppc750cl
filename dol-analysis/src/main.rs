use std::fs::File;

use clap::{Arg, ArgMatches, Command};
use dol::Dol;

mod init_section;
mod text_section;

use crate::{init_section::*, text_section::*};

fn main() {
    let matches = Command::new("dol-analysis")
        .arg(Arg::new("dol").takes_value(true).required(true))
        .arg(Arg::new("--text-start").takes_value(true))
        .arg(Arg::new("--text-stop").takes_value(true))
        .get_matches();
    if let Err(err) = _main(matches) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T> = std::result::Result<T, Error>;

fn _main(matches: ArgMatches) -> Result<()> {
    let dol_path = matches.value_of("dol").expect("no DOL");
    let dol = Dol::read_from(File::open(dol_path)?)?;

    let init = InitInfo::from_dol(&dol)?;
    println!("{:?}", init);

    let arb_text_addr = *init.text_call_targets.last().unwrap();
    let text = TextInfo::from_dol(&dol, arb_text_addr)?;
    println!("{:?}", text);

    Ok(())
}
