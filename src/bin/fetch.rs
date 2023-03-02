//! # fetch.rs  -- asset fetching from asset server
//
//  Animats
//  February, 2022
//
//  Loader for mesh and sculpt assets.
//  Called from threads in the asset load thread pool.
//

use anyhow::{Context, Error};
use ureq::{Agent, AgentBuilder};
use std::time::{Duration};

/// Fetch asset from asset server.
pub fn fetch_asset(
    agent: &mut Agent,
    url: &str,
    byte_range_opt: Option<(u32, u32)>)
    -> Result<Vec<u8>, Error> {
    //  HTTP read
    let query = if let Some(byte_range) = byte_range_opt {
        agent
            .get(&url)
            .set("Range", format!("bytes={}-{}",byte_range.0, byte_range.1).as_str())
    } else {
        agent
            .get(&url)
    };
            
    let resp = 
        query
            .call()
            .map_err(anyhow::Error::msg)?;
    let mut buffer = Vec::new();
    resp.into_reader().read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Build user agent for queries.
pub fn build_agent(user_agent: &str) -> Agent {
    const NETWORK_TIMEOUT: Duration = Duration::from_secs(15);  // something has gone wrong
    AgentBuilder::new()
        .user_agent(user_agent)
        .timeout_connect(NETWORK_TIMEOUT)
        .timeout_read(NETWORK_TIMEOUT)
        .timeout_write(NETWORK_TIMEOUT)
        .build()
}
