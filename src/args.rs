use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum ConversionFormat {
    Mp3,
    Flac,
}

impl std::fmt::Display for ConversionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionFormat::Mp3 => write!(f, "mp3"),
            ConversionFormat::Flac => write!(f, "flac"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input audiobook file (positional argument)
    #[arg(value_name = "INPUT")]
    pub input: String,

    /// Output directory (optional, default is input_name + "_chapters")
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Keep m4b format
    #[arg(short = 'k', long = "keep", default_value_t = false)]
    pub no_convert: bool,

    // Format to convert m4b files to, mp3 or flac
    #[arg(short = 'f', long = "conversion-format", default_value_t = ConversionFormat::Mp3)]
    pub conversion_format: ConversionFormat,

    /// Conversion quality (1=best, 9=worst)
    #[arg(short = 'q', long, default_value_t = 2)]
    pub quality: u8,

    /// Sanitize filenames (default: false)
    /// This option replaces invalid characters with underscores
    #[arg(short = 's', long = "sanitize", default_value_t = false)]
    pub sanitize: bool,
}
