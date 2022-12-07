use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub command_type: PngSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PngSubcommand {
    Encode(EncodePng),
    Decode(DecodePng),
    Remove(RemovePng),
    Print(PrintPng)
}

#[derive(Debug, Args)]
pub struct EncodePng {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_path: Option<String>
}

#[derive(Debug, Args)]
pub struct DecodePng {
    pub file_path: String,
    pub chunk_type: String
}

#[derive(Debug, Args)]
pub struct RemovePng {
    pub file_path: String,
    pub chunk_type: String
}

#[derive(Debug, Args)]
pub struct PrintPng {
    pub file_path: String
}