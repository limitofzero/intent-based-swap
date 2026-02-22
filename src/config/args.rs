use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(long, env = "RPC_URL", default_value = "")]
    pub rpc_url: String,
}

impl Args {
    pub fn from_env() -> Self {
        Self::parse()
    }
}
