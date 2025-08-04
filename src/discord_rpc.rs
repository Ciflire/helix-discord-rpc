use std::path::PathBuf;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use steel::rvals::Custom;

pub struct DiscordRPC {
    workspace: PathBuf,
    file: PathBuf,
    ipc_client: DiscordIpcClient,
}

impl Custom for DiscordRPC {}

impl DiscordRPC {
    pub fn new(workspace: String, file: String) -> DiscordRPC {
        DiscordRPC {
            workspace: workspace.into(),
            file: file.into(),
            ipc_client: DiscordIpcClient::new("1173288863217766421").unwrap(),
        }
    }

    pub fn connect(&mut self) {
        let _ = self.ipc_client.connect();
    }
}
