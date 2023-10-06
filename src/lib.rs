use std::borrow::Cow;
use std::fmt::{self, Formatter};
use std::future::Future;
use std::panic;
use std::pin::Pin;

use magic_wormhole::{AppConfig as WhAppConfig, AppID, Code, Wormhole as Wh, WormholeError as WhError};
use magic_wormhole::transfer::AppVersion;
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

#[wasm_bindgen]
/// Represents the awaitable handshake future that the `Wormhole::connect_without_code` function returns.
pub struct Handshake(Pin<Box<dyn Future<Output=Result<Wh, WhError>>>>);

#[wasm_bindgen]
/// Represents the tuple containing the `WormholeWelcome` and the awaitable handshake future that the `Wormhole::connect_without_code`
/// function returns.
pub struct WelcomeAndHandshake(WormholeWelcome, Handshake);

#[wasm_bindgen]
/// Represents the tuple containing the `WormholeWelcome` and the `Wormhole` object that the `Wormhole::connect_with_code`
/// function returns.
pub struct WelcomeAndWormhole(WormholeWelcome, Wh);

#[wasm_bindgen]
impl Wormhole {
    /// Generates a core wormhole AppConfig from the provided custom AppConfig.
    fn get_wh_config(config: &AppConfig) -> WhAppConfig<AppVersion> {
        WhAppConfig {
            id: AppID(Cow::from(config.id.clone())),
            rendezvous_url: Cow::from(config.rendezvous_url.clone()),
            app_version: AppVersion {},
        }
    }

    #[wasm_bindgen]
    /// Generate a code and connect to the rendezvous server.
    ///
    /// It returns the "welcome" from the server along with the awaitable handshake.
    ///
    /// # Arguments
    ///
    /// * `config` - The app configuration.
    /// * `code_length` - The number of words to include in the generated wormhole code.
    pub async fn connect_without_code(config: &AppConfig, code_length: usize) -> Result<WelcomeAndHandshake, WormholeError> {
        let config = Self::get_wh_config(&config);
        let (welcome, handshake) = Wh::connect_without_code(config, code_length).await?;

        Ok(WelcomeAndHandshake(
            WormholeWelcome {
                welcome: welcome.welcome,
                code: welcome.code.0,
            },
            Handshake(Box::pin(handshake)),
        ))
    }

    #[wasm_bindgen]
    /// Connect to a peer with a code.
    ///
    /// It returns the "welcome" from the server along with the wormhole object.
    ///
    /// # Arguments
    ///
    /// * `config` - The app configuration.
    /// * `code` - The wormhole code.
    /// * `expect_claimed_nameplate` - Whether or not to expect a claimed nameplate. Defaults to `false`.
    pub async fn connect_with_code(config: &AppConfig, code: &str, expect_claimed_nameplate: Option<bool>) -> Result<WelcomeAndWormhole, WormholeError> {
        let expect_claimed_nameplate = expect_claimed_nameplate.unwrap_or(false);
        let config = Self::get_wh_config(&config);
        let (welcome, wh) = Wh::connect_with_code(config, Code(code.to_string()), expect_claimed_nameplate).await?;

        Ok(WelcomeAndWormhole(
            WormholeWelcome {
                welcome: welcome.welcome,
                code: welcome.code.0,
            },
            wh,
        ))
    }
}
