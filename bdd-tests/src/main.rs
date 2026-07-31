// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod config;
mod logging;
mod steps;
mod world;

use cucumber::World;

use config::TestSettings;
use logging::init_logging;
use world::GvmdWorld;

#[tokio::main]
async fn main() {
    let settings = TestSettings::load().expect("failed to load BDD test settings");

    init_logging(&settings.log_level);

    GvmdWorld::cucumber()
        .fail_on_skipped()
        .run("bdd-tests/features")
        .await;
}
