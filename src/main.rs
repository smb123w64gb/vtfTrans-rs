
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Converts XTF to VTF
    #[arg(short,long)]
    vtf: bool,
    /// Converts VTF to XTF
    #[arg(short,long)]
    xtf: bool,

    /// Takes in .vtf or .xtf to convert from
    #[arg(value_name = "FILE")]
    input_file: Option<PathBuf>,
    /// Takes in .vtf or .xtf to convert to
    #[arg(value_name = "FILE")]
    output_file: Option<PathBuf>,

}

mod vtf;
mod image_format;

fn main() {
    let args = Args::parse();
    if args.vtf {
        match args.input_file {
            Some( input) => {
            println!("File {} inputed",input.display());
            let mut output:PathBuf;
            match args.output_file {
                Some(out) => output = out,
                None => {output = input.clone();
                    output.set_extension("vtf");
                },
            }
            println!("Output file is {}",output.display())
        },
            None => println!("Can not progress\n No file found for input"),
            
        }
    } 

}
