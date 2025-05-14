use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "parse_firebase",
    about = "Parse Firebase JSON data for SSBU Training Modpack"
)]
struct Opt {
    /// Input file path
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file path
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    training_mod_metrics::parser::extract_smash_open_devices(&opt.input, &opt.output)?;
    Ok(())
}
