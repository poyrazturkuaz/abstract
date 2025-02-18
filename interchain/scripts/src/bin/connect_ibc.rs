use abstract_interface::Abstract;
use abstract_scripts::abstract_ibc::abstract_ibc_connection_with;
use abstract_scripts::{NEUTRON_1, ROLLKIT_TESTNET};
use cw_orch::daemon::networks::{ARCHWAY_1, JUNO_1, OSMO_5, PHOENIX_1};
use cw_orch::prelude::*;
use cw_orch::tokio::runtime::Handle;
use cw_orch_polytone::Polytone;
use tokio::runtime::Runtime;

/// Connect IBC between two chains.
/// @TODO update this to take in the networks as arguments.
fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    env_logger::init();

    let chains = vec![
        (ROLLKIT_TESTNET, None),
        (OSMO_5, None),
        (JUNO_1, None),
        (PHOENIX_1, None),
        (ARCHWAY_1, None),
        (NEUTRON_1, None),
        // (OSMOSIS_1, Some(std::env::var("OSMOSIS_MNEMONIC")?)),
    ];
    let runtime = Runtime::new()?;

    let src_chain = &chains[1];
    let dst_chain = &chains[0];

    connect(src_chain.clone(), dst_chain.clone(), runtime.handle())?;

    Ok(())
}

fn get_daemon(
    chain: ChainInfo,
    handle: &Handle,
    mnemonic: Option<String>,
    deployment_id: Option<String>,
) -> cw_orch::anyhow::Result<Daemon> {
    let mut builder = DaemonBuilder::default();
    builder.chain(chain).handle(handle);
    if let Some(mnemonic) = mnemonic {
        builder.mnemonic(mnemonic);
    }
    if let Some(deployment_id) = deployment_id {
        builder.deployment_id(deployment_id);
    }
    Ok(builder.build()?)
}

pub fn get_deployment_id(src_chain: &ChainInfo, dst_chain: &ChainInfo) -> String {
    format!("{}-->{}", src_chain.chain_id, dst_chain.chain_id)
}

fn connect(
    (src_chain, src_mnemonic): (ChainInfo, Option<String>),
    (dst_chain, dst_mnemonic): (ChainInfo, Option<String>),
    handle: &Handle,
) -> cw_orch::anyhow::Result<()> {
    let src_daemon = get_daemon(src_chain.clone(), handle, src_mnemonic.clone(), None)?;
    let dst_daemon = get_daemon(dst_chain.clone(), handle, dst_mnemonic, None)?;

    let src_abstract = Abstract::load_from(src_daemon.clone())?;
    let dst_abstract = Abstract::load_from(dst_daemon.clone())?;

    let src_polytone_daemon = get_daemon(
        src_chain.clone(),
        handle,
        src_mnemonic,
        Some(get_deployment_id(&src_chain, &dst_chain)),
    )?;

    let src_polytone = Polytone::load_from(src_polytone_daemon)?;

    let interchain = DaemonInterchainEnv::from_daemons(
        handle,
        vec![src_daemon, dst_daemon],
        &ChannelCreationValidator,
    );

    abstract_ibc_connection_with(&src_abstract, &interchain, &dst_abstract, &src_polytone)?;

    Ok(())
}
