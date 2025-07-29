use clap::Parser;

/// Prints an ASCII art banner to look cool!
pub fn banner() {
  println!("{} {}\n", include_str!("banner"), env!("CARGO_PKG_VERSION"))
}

#[derive(Parser, Debug)]
#[clap(
  name = "binserve",
  about = "A fast static web server with Automatic HTTPs, routing, templating, and security in a single binary you can setup with zero code.",
  author = "Mufeed VH <mufeed@lyminal.space>"
  // disable_help_flag = true
)]
pub struct Cli {
  #[clap(
    long = "host",
    // Command binserve: Short option names must be unique for each argument, but '-h' is in use by both 'host' and 'help' (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)
    // short = 'h',
    required = false,
    help = "Host to run binserve on.",
    value_name = "HOST IP/DOMAIN:PORT"
  )]
  pub host: Option<String>,
  #[clap(
    long = "key",
    short = 'k',
    value_name = "TLS KEY",
    help = "TLS key file.",
    required = false
  )]
  pub tls_key: Option<String>,
  #[clap(
    short = 'c',
    long = "cert",
    value_name = "TLS CERT",
    help = "TLS cert file.",
    required = false
  )]
  pub tls_cert: Option<String>,
  #[clap(
    required = false,
    help = "Path to the binserve configuration file.",
    value_name = "CONFIG PATH",
    long = "config_path",
    short = 'p'
  )]
  pub config_path: Option<String>,
}

/* // move to [Cli]
/// Command-line arguments
pub fn args() -> ArgMatches {
  Command::new("binserve")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Mufeed VH <mufeed@lyminal.space>")
    .about(
      "A fast static web server with Automatic HTTPs, routing, templating, and security in a single binary you can setup with zero code."
    )
    // .arg(Arg::new("command").help("Command to run.").value_name("COMMAND").required(false).index(1))
    .arg(
      Arg::new("host")
        .short('h')
        .long("host")
        .value_name("HOST IP/DOMAIN:PORT")
        .help("Host to run binserve on.")
        .required(false)
    )
    .arg(
      Arg::new("tls_key")
        .short('k')
        .long("key")
        .value_name("TLS KEY")
        .help("TLS key file.")
        .required(false)
    )
    .arg(
      Arg::new("tls_cert")
        .short('c')
        .long("cert")
        .value_name("TLS CERT")
        .help("TLS cert file.")
        .required(false)
    )
    .arg(Arg::new("help").long("help").action(ArgAction::HelpLong))
    .disable_help_flag(true)
    .get_matches()
}
*/
