use std::fmt::{self, Formatter};

use magic_wormhole::WormholeError as WhError;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Wormhole configuration corresponding to an upper layer protocol
///
/// There are multiple different protocols built on top of the core Wormhole protocol.
/// They are identified by a unique URI-like ID string, an URL to find the rendezvous server (might be shared among multiple protocols),
/// and client implementations also have a “version” data to do protocol negotiation.
pub struct AppConfig {
    id: String,
    rendezvous_url: String,
    // Placeholder, till I can figure out how to pass this to the actual wormhole config.
    _app_version: serde_json::Value,
}

#[derive(Error, Debug)]
pub struct WormholeError(#[from] WhError);

impl fmt::Display for WormholeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<WormholeError> for JsValue {
    fn from(value: WormholeError) -> Self {
        value.to_string().into()
    }
}

#[wasm_bindgen]
/// The result of the client-server handshake.
pub struct WormholeWelcome {
    /// A welcome message from the server (think of “message of the day”). Should be displayed to the user if present.
    welcome: Option<String>,
    /// The wormhole code.
    code: String,
}

#[wasm_bindgen]
/// Establishing Wormhole connection
pub struct Wormhole;
