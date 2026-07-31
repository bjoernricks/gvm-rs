// SPDX-FileCopyrightText: 2026 Björn Ricks <bjoern.ricks@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use gvm_rs::commands::authenticate::AuthenticateResponse;

#[derive(Debug, Default)]
pub struct AuthenticationState {
    pub response: Option<AuthenticateResponse>,
}