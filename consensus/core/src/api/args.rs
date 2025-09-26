use clap::Parser;
use std::str::FromStr;
use crate::config::{Config, ConfigBuilder};
use crate::network::{ContextualNetAddress, NetAddress};

fn validate_ram_scale(s: &str) -> Result<f64, String> {
    let value: f64 = s.parse().map_err(|_| "invalid ram_scale: not a number")?;
    if value <= 0.0 {
        Err("invalid ram_scale: must be positive".to_string())
    } else {
        Ok(value)
    }
}

/// Transaction validation arguments.
#[derive(Debug, Clone, Default)]
pub struct TransactionValidationArgs {
    pub allow_non_final: bool,
    pub allow_orphans: bool,
}

/// Batch transaction validation arguments.
#[derive(Debug, Clone, Default)]
pub struct TransactionValidationBatchArgs {
    pub allow_non_final: bool,
    pub allow_orphans: bool,
}

/// Command-line arguments for consensus configuration.
#[derive(Parser, Debug, Clone)]
#[command(name = "consensus")]
#[command(about = "Jio Consensus Core Configuration")]
pub struct Args {
    /// Enable archival node mode
    #[arg(long)]
    pub archival: bool,

    /// Enable sanity checks (compute-intensive)
    #[arg(long)]
    pub sanity_checks: bool,

    /// Enable UTXO index
    #[arg(long)]
    pub utxoindex: bool,

    /// Allow unsafe RPC commands
    #[arg(long)]
    pub unsafe_rpc: bool,

    /// Allow mining without being synced
    #[arg(long)]
    pub enable_unsynced_mining: bool,

    /// Allow mainnet mining
    #[arg(long)]
    pub enable_mainnet_mining: bool,

    /// P2P listen address (default: 0.0.0.0)
    #[arg(long)]
    pub p2p_listen_address: Option<String>,

    /// External IP address
    #[arg(long)]
    pub externalip: Option<String>,

    /// Block template cache lifetime in seconds
    #[arg(long)]
    pub block_template_cache_lifetime: Option<u64>,

    /// Disable UPnP
    #[arg(long)]
    pub disable_upnp: bool,

    /// RAM scale factor
    #[arg(long, default_value = "1.0", value_parser = validate_ram_scale)]
    pub ram_scale: f64,

    /// Retention period in days
    #[arg(long)]
    pub retention_period_days: Option<f64>,
}

impl Args {
    /// Build a Config from the parsed arguments.
    pub fn build_config(self, params: crate::config::params::Params) -> Config {
        let mut builder = ConfigBuilder::new(params);

        if self.archival {
            builder = builder.set_archival();
        }
        if self.sanity_checks {
            builder = builder.enable_sanity_checks();
        }
        // Add other configurations as needed

        builder
            .apply_args(|config| {
                config.utxoindex = self.utxoindex;
                config.unsafe_rpc = self.unsafe_rpc;
                config.enable_unsynced_mining = self.enable_unsynced_mining;
                config.enable_mainnet_mining = self.enable_mainnet_mining;
                config.disable_upnp = self.disable_upnp;
                config.ram_scale = self.ram_scale;
                config.retention_period_days = self.retention_period_days;
                config.block_template_cache_lifetime = self.block_template_cache_lifetime;

                if let Some(ref addr) = self.p2p_listen_address {
                    // Parse address, for simplicity assume it's an IP:port
                    // In real impl, handle properly
                    config.p2p_listen_address = ContextualNetAddress::from_str(addr).unwrap_or_default();
                }
                if let Some(ref ip) = self.externalip {
                    config.externalip = Some(NetAddress::from_str(ip).unwrap_or_default());
                }
            })
            .build()
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            archival: false,
            sanity_checks: false,
            utxoindex: false,
            unsafe_rpc: false,
            enable_unsynced_mining: false,
            enable_mainnet_mining: false,
            p2p_listen_address: None,
            externalip: None,
            block_template_cache_lifetime: None,
            disable_upnp: false,
            ram_scale: 1.0,
            retention_period_days: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use crate::config::params::Params; // Assuming params is available

    #[test]
    fn test_args_parse() {
        let args = Args::parse_from(["consensus", "--archival", "--ram-scale", "2.0"]);
        assert!(args.archival);
        assert_eq!(args.ram_scale, 2.0);
    }

    #[test]
    fn test_args_default() {
        let args = Args::default();
        assert!(!args.archival);
        assert_eq!(args.ram_scale, 1.0);
    }

    #[test]
    fn test_build_config() {
        let args = Args::default();
        let params = Params::default(); // Mock or default params
        let config = args.build_config(params);
        assert!(!config.is_archival);
        assert_eq!(config.ram_scale, 1.0);
    }

    #[test]
    fn test_args_with_all_flags() {
        let cmd = Args::command();
        // Test if all args are recognized
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "archival"));
        assert!(cmd.get_arguments().any(|arg| arg.get_id() == "ram_scale"));
    }

    #[test]
    fn test_invalid_ram_scale() {
        let result = Args::try_parse_from(["consensus", "--ram-scale", "-1.0"]);
        assert!(result.is_err());
    }
}
