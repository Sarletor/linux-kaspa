use crate::cli::Opt;
use crate::client::KaspadHandler;
use crate::proto::NotifyBlockAddedRequestMessage;
use std::error::Error as StdError;
use std::fmt;
use structopt::StructOpt;

mod cli;
mod client;
mod kaspad_messages;
mod miner;
mod pow;
mod target;

pub mod proto {
    tonic::include_proto!("protowire");
    // include!("protowire.rs"); // FIXME: https://github.com/intellij-rust/intellij-rust/issues/6579
}

pub type Error = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Copy, Clone)]
pub struct Hash(pub [u8; 32]);

impl fmt::LowerHex for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|&c| write!(f, "{:02x}", c))
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut opt: Opt = Opt::from_args();
    opt.process()?;
    env_logger::builder().filter_level(opt.log_level()).parse_default_env().init();

    let mut client = KaspadHandler::connect(opt.kaspad_address, opt.mining_address).await?;

    client.client_send(NotifyBlockAddedRequestMessage {}).await?;

    client.listen(opt.num_threads).await
}
