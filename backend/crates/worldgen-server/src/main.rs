use clap::Parser;

use worldgen_api::{CreateWorldRequest, WorldSeed};
use worldgen_core::WorldStore;

mod tools;

#[derive(Debug, Parser)]
#[command(
    name = "worldgen-server",
    version,
    about = "Rust backend for WorldGen MCP"
)]
struct Args {
    #[arg(long)]
    print_contract: bool,
}

fn main() {
    let args = Args::parse();

    if args.print_contract {
        println!(
            "{}",
            tools::describe_contract()
        );
        return;
    }

    let store = WorldStore::new();
    let request = CreateWorldRequest {
        request_id: "bootstrap".to_string(),
        seed: WorldSeed {
            value: "default".to_string(),
            world_version: "0.1".to_string(),
            salt: None,
        },
        world_name: "WorldGen Sandbox".to_string(),
        profile_name: "default".to_string(),
        world_size: 4096,
        chunk_size: 64,
        enable_streaming: true,
        assetless_mode: true,
        generation_version: "0.1".to_string(),
    };

    let mut router = tools::ToolRouter::new(store);
    let response = router.create_world(request);
    println!(
        "{}",
        serde_json::to_string_pretty(&response).unwrap_or_default()
    );
}
