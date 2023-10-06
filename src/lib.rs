use std::fmt::{self, Formatter};
use std::panic;

use magic_wormhole::WormholeError as WhError;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
/// Runs initialization stuff for the module.
///
/// This function will execute automatically, and need not (and ideally should not) be called manually.
pub fn _init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

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

#[wasm_bindgen]
impl AppConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(id: String, rendezvous_url: String) -> Self {
        Self {
            id,
            rendezvous_url,
            // This is currently a placeholder field, so the value doesn't really matter.
            _app_version: "".into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[wasm_bindgen(getter)]
    pub fn rendezvous_url(&self) -> String {
        self.rendezvous_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_rendezvous_url(&mut self, rendezvous_url: String) {
        self.rendezvous_url = rendezvous_url;
    }
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
impl WormholeWelcome {
    #[wasm_bindgen(getter)]
    pub fn welcome(&self) -> String {
        self.welcome.clone().unwrap_or("".into())
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }
}

#[wasm_bindgen]
/// Establishing Wormhole connection.
pub struct Wormhole;
