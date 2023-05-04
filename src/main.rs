use clap::Parser;
use vt3::VtClient;

#[derive(Parser)]
#[command(name = "Drtrctrv")]
#[command(author = "Guillaume B. <guillaumdboehm@hotmail.fr>")]
#[command(version = "1.0")]
#[command(about = "Inspects files in the event that bad people send you bad files for bad reasons", long_about = None)]
struct Cli {
    filename: std::path::PathBuf,

    /// Don't send the file to virustotal
    #[arg(short='v', long)]
    novirustotal: bool,

    /// Use a specific virustotal API key (takes precedence over --vtapikeyfile)
    #[arg(short='k', long, value_name = "API_KEY")]
    vtapikey: Option<String>,

    /// Use a diferent file to look for the virustotal API key
    #[arg(short='K', long, value_name = "API_KEY_FILENAME")]
    vtapikeyfile: Option<std::path::PathBuf>,

    /// Don't look at the file type
    #[arg(short='f', long)]
    nofiletype: bool,
}

fn main() {
    let args = Cli::parse();

    // Default values
    let default_vt_api_key_file = "~/.config/secret/drtrctrv.vtapikey";
}
