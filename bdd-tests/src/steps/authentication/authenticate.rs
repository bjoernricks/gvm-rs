// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use cucumber::{then, when};
use gvm_rs::commands::authenticate::{
    AuthenticateRequest,
    AuthenticateResponse,
};

use crate::world::GvmdWorld;

#[when("I authenticate with the configured credentials")]
fn authenticate_with_configured_credentials(world: &mut GvmdWorld) {
    let client = world
        .client
        .as_mut()
        .expect("GMP client is not connected");

    let request = AuthenticateRequest::new(
        &world.settings.username,
        &world.settings.password,
    );

    client
        .send_command(&request)
        .expect("failed to send the authenticate command");

    let response = client
        .receive_response::<AuthenticateResponse>()
        .expect("failed to receive the authentication response");

    world.authentication.response = Some(response);
}

#[then("the authentication should succeed")]
fn authentication_should_succeed(world: &mut GvmdWorld) {
    let response = world
        .authentication
        .response
        .as_ref()
        .expect("authentication response is not available");

    assert_eq!(
        response.status,
        200,
        "authentication failed with status {}: {}",
        response.status,
        response.status_text
    );
}

#[then("an authenticated role should be returned")]
fn authenticated_role_should_be_returned(world: &mut GvmdWorld) {
    let response = world
        .authentication
        .response
        .as_ref()
        .expect("authentication response is not available");

    assert!(
        !response.role.trim().is_empty(),
        "authentication role must not be empty"
    );
}