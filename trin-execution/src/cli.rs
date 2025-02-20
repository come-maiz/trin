use std::{net::SocketAddr, path::PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::types::block_to_trace::BlockToTrace;

#[derive(Parser, Debug, Clone)]
#[command(name = "Trin Execution", about = "Executing blocks with no devp2p")]
pub struct TrinExecutionConfig {
    #[arg(
        short = 'e',
        long = "ephemeral",
        help = "Use temporary data storage that is deleted on exit."
    )]
    pub ephemeral: bool,

    #[arg(
        long,
        default_value = "none",
        help = "The block traces will be dumped to the working directory: Configuration options ['none', 'block:<number>', 'all']."
    )]
    pub block_to_trace: BlockToTrace,

    #[arg(
        long = "enable-metrics-with-url",
        help = "Enable prometheus metrics reporting (provide local IP/Port from which your Prometheus server is configured to fetch metrics)"
    )]
    pub enable_metrics_with_url: Option<SocketAddr>,

    #[command(subcommand)]
    pub command: Option<TrinExecutionSubCommands>,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum TrinExecutionSubCommands {
    /// Import a era2 state snapshot from a file, useful for bootstrapping a new node quickly
    ImportState(ImportStateConfig),
    /// Export the current state of the node to a era2 file
    ExportState(ExportStateConfig),
}

#[derive(Args, Debug, Default, Clone, PartialEq)]
pub struct ImportStateConfig {
    #[arg(
        long = "path-to-era2",
        help = "path to where the era2 state snapshot is located"
    )]
    pub path_to_era2: PathBuf,
}

#[derive(Args, Debug, Default, Clone, PartialEq)]
pub struct ExportStateConfig {
    #[arg(
        long = "path-to-era2",
        help = "path to where the era2 state snapshot is located"
    )]
    pub path_to_era2: PathBuf,
}
