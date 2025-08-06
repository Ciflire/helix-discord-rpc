use steel::{
    declare_module,
    steel_vm::ffi::{FFIModule, RegisterFFIFn as _},
};

use crate::discord_rpc::DiscordRPC;

declare_module!(helix_discord_rpc_steel_module);

/// Declare the Steel module which will be dynamically loaded
fn helix_discord_rpc_steel_module() -> FFIModule {
    let mut module = FFIModule::new("steel/helix-discord-rpc");

    module
        .register_fn("DiscordRPC::new", DiscordRPC::new)
        .register_fn("DiscordRPC::connect", DiscordRPC::connect)
        .register_fn("DiscordRPC::set_activity", DiscordRPC::set_activity)
        .register_fn("DiscordRPC::set_idle", DiscordRPC::set_idle);

    module
}
