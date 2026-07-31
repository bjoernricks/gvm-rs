use cucumber::{given, then};
use gvm_rs::client::GmpClient;

use crate::world::GvmdWorld;

#[given("the local gvmd Unix socket is available")]
fn local_gea_unix_socket_is_available(world: &mut GvmdWorld) {
    let socket_path = &world.settings.socket_path;

    let client = GmpClient::from_unix_socket_path(socket_path).unwrap_or_else(|error| {
        panic!(
            "failed to connect to the gvmd Unix socket at '{}': {error}",
            socket_path.display()
        )
    });

    world.client = Some(client);
}

#[then("the GMP client should be connected")]
fn gmp_client_should_be_connected(world: &mut GvmdWorld) {
    assert!(
        world.client.is_some(),
        "expected the GMP client to be connected"
    );
}