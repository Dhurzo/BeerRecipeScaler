use clap::Parser;

/// Beer Recipe Scaler CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Number of hop additions in recipe
    #[arg(short = 'p', long)]
    pub hop_number: usize,

    /// Number of malt additions in recipe
    #[arg(short = 'm', long)]
    pub malt_number: usize,

    /// Original recipe volume in liters
    #[arg(short = 'o', long)]
    pub original_volume: f32,

    /// Scaled recipe volume in liters
    #[arg(short = 's', long)]
    pub scaled_volume: f32,

    /// Scale by "malt" or "volume"
    #[arg(short = 'b', long, default_value = "volume")]
    pub scale_by: String,
}

/// Checks validity of command line arguments
pub fn check_arguments(args: &Args) {
    if args.malt_number == 0 {
        panic!("No malts, no recipe");
    }
    if args.original_volume <= 0.0 {
        panic!("No volume, no recipe");
    }
    if args.scaled_volume <= 0.0 && args.scale_by != "malt" {
        panic!("No volume with scale by volume, no recipe");
    }
}