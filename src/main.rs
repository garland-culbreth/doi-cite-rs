use clap::Parser;
use ureq::Error;

/// Get a formatted citation from a DOI identifier
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// DOI identifier to cite
    #[arg(short, long)]
    doi: String,
    /// Citation style to use
    #[arg(short, long, default_value_t = String::from("apa"))]
    style: String,
    /// Citation language to use
    #[arg(short, long, default_value_t = String::from("en-US"))]
    lang: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let url = format!(
        "https://citation.doi.org/format?doi={}&style={}&lang={}",
        args.doi, args.style, args.lang
    );

    match ureq::get(url).call() {
        Ok(mut response) => {
            println!("{}", response.body_mut().read_to_string()?)
        },
        Err(Error::StatusCode(code)) => {
            panic!("Server error, code: {}", code)
        }
        Err(error) => {
            panic!("Unable to retrieve citation: {}", error)
        }
    }

    Ok(())
}
