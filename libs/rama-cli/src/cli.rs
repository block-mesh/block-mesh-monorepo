use crate::server::HttpVersion;
use clap::Parser;
use rama::cli::ForwardKind;

#[derive(Debug, Parser)]
/// rama fp service (used for  collection in purpose of UA emulation)
pub struct CliCommand {
    #[arg(short = 'p', long, default_value_t = 443)]
    /// the port to listen on
    pub port: u16,

    #[arg(short = 'i', long, default_value = "0.0.0.0")]
    /// the interface to listen on
    pub interface: String,

    #[arg(short = 'c', long, default_value_t = 0)]
    /// the number of concurrent connections to allow
    ///
    /// (0 = no limit)
    pub concurrent: usize,

    #[arg(short = 't', long, default_value_t = 8)]
    /// the timeout in seconds for each connection
    ///
    /// (0 = no timeout)
    pub timeout: u64,

    #[arg(long, short = 'f')]
    /// enable support for one of the following "forward" headers or protocols
    ///
    /// Supported headers:
    ///
    /// Forwarded ("for="), X-Forwarded-For
    ///
    /// X-Client-IP Client-IP, X-Real-IP
    ///
    /// CF-Connecting-IP, True-Client-IP
    ///
    /// Or using HaProxy protocol.
    pub forward: Option<ForwardKind>,

    /// http version to serve  Service from
    #[arg(long, default_value = "auto")]
    pub http_version: HttpVersion,

    #[arg(long, short = 's')]
    /// run echo service in secure mode (enable TLS)
    pub secure: bool,

    #[arg(long)]
    /// use self-signed certs in case secure is enabled
    pub self_signed: bool,
}
