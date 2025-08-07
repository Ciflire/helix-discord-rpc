use std::path::Path;
use std::path::PathBuf;

use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{self, Activity, Assets},
};
use steel::rvals::Custom;

use crate::assets::get_asset;

pub struct DiscordRPC {
    ipc_client: DiscordIpcClient,
}

impl Custom for DiscordRPC {}

/// Implementation of the functionnalities we need to use in steel
impl DiscordRPC {
    /// Creates and return the "server" as it is called in the steel module
    /// This object stays alive as long as it is assigned to a steel variable
    pub fn new() -> DiscordRPC {
        DiscordRPC {
            ipc_client: DiscordIpcClient::new("1173288863217766421").unwrap(),
        }
    }

    /// This connects the "server" to discord, it binds it to the correct port
    pub fn connect(&mut self) {
        let _ = self.ipc_client.connect();
    }

    /// Sets the activity with the correct language icon
    pub fn set_activity(&mut self, path: String, workspace: String, row: usize, col: usize) {
        let binding = path.clone();
        let filename = Path::new(&binding).file_name().unwrap().to_str().unwrap();
        let state = "Editing ".to_string()
            + filename
            + " "
            + &row.to_string()
            + ":"
            + &(col - 6).to_string();
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

    /// Sets the idle status (Not used at the moment)
    pub fn set_idle(&mut self) {
        let activity = Activity::new().assets(Assets::new().small_image("idle"));
        let _ = self.ipc_client.set_activity(activity);
    }
}
