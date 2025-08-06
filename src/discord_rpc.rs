use std::path::{Path, PathBuf};

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{self, Activity, Assets},
};
use steel::rvals::Custom;

use crate::assets::{self, get_asset};

pub struct DiscordRPC {
    ipc_client: DiscordIpcClient,
}

impl Custom for DiscordRPC {}

impl DiscordRPC {
    pub fn new() -> DiscordRPC {
        DiscordRPC {
            ipc_client: DiscordIpcClient::new("1173288863217766421").unwrap(),
        }
    }

    pub fn connect(&mut self) {
        let _ = self.ipc_client.connect();
    }

    pub fn set_activity(&mut self, path: String, workspace: String) {
        let binding = path.clone();
        let filename = Path::new(&binding).file_name().unwrap().to_str().unwrap();
        let state = "Editing ".to_string() + filename;
        let binding = workspace.clone();
        let folder = Path::new(&binding).file_name().unwrap().to_str().unwrap();
        let details = "Workspace ".to_string() + folder;
        let binding = get_asset(filename.to_string());
        let activity = activity::Activity::new()
            .state(&state)
            .details(&details)
            .assets(
                Assets::new()
                    .large_image(&binding)
                    .small_image("helix")
                    .small_text("https://github.com/helix-editor/helix"),
            );
        let _ = self.ipc_client.set_activity(activity);
    }

    pub fn set_idle(&mut self) {
        let activity = Activity::new().assets(Assets::new().small_image("idle"));
        let _ = self.ipc_client.set_activity(activity);
    }
}
