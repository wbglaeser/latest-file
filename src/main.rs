use structopt::StructOpt;
use latest_file::{FileEntry, parse_dir, Cli};

fn main() {
   
    // setup arguments and file tracker
    let args = Cli::from_args();
    let mut latest_file = FileEntry::new();

    // recursively scan directories
    parse_dir(args.path, &mut latest_file, &args.exclude, args.faulty_files);

    // print latest file
    println!("{}", latest_file);

}
