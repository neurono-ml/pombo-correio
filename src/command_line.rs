use clap::Parser;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct CommandLine {
    /// Hub configuration file
    #[clap(long, short='f', env="HUB_CONFIGURATION_FILE")]
    pub hub_configuration_file: String,

    /// The host where service will listen
    #[clap(long, default_value="0.0.0.0", env="HOST")]
    pub host: String,

    /// The port where service will listen
    #[clap(short, long, default_value="8080", env="PORT")]
    pub port: u16,
}