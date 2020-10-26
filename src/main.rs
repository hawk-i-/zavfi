use clap::Clap;

#[derive(Clap)]
#[clap(version="1.0", author="Gurbakhshish Singh")]
struct Opts{

    #[clap(short, long, default_value="config.yaml")]
    config: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{config_file_name}", config_file_name = opts.config)
}
