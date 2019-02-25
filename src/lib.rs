#![allow(non_snake_case)]

/*!
Handles the oauth2 flow for installed apps that don't have a server side

```
fn main() -> Result<(), Box<std::error::Error>> {
    let config = loadConfig()?;

    let auth_url = "https://accounts.google.com/o/oauth2/v2/auth";
    let token_url = "https://www.googleapis.com/oauth2/v3/token";

    // Set up the config for the Google OAuth2 process.
    let oauthConfig = Oauth2Config::new(
        config.googleapi_app_credentials.client_id,
        config.googleapi_app_credentials.client_secret,
        auth_url,
        token_url,
    )
    .add_scope("https://www.googleapis.com/auth/calendar")
    .add_scope("https://www.googleapis.com/auth/plus.me");

    const PORT: u16 = 14565;
    let authenticator = oauth2_noserver::Authenticator::new(oauthConfig)
        .set_port(PORT)
        .set_redirect_url(format!("http://localhost:{}/oauth/callback", PORT));
    authenticator.authenticate().unwrap();

    Ok(())
}
```
*/

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

pub use oauth2::Config;
use rand::Rng;
use url::Url;
use webbrowser;

/// Returns a new Flaker based on the specified identifier
///
/// # Arguments
///
/// * `identifier` - A 6 byte vec that provides some arbitrary identification.
///
/// # Remarks
///
/// This is a convenience function that converts the `identifier` `vec` into
/// a 6 byte array. Where possible, prefer the array and use `new`.
///
/// *Note*: This also assumes the `flaker` is being created on a little endian
/// CPU.
pub struct Authenticator {
    oauthConfig: oauth2::Config,
    port: u16,
}
impl Authenticator {
    pub fn new(oauthConfig: oauth2::Config) -> Self {
        Authenticator {
            oauthConfig: oauthConfig
                .set_redirect_url("http://localhost:14565/oauth/callback".to_string()),
            port: 14565,
        }
    }
    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    pub fn set_redirect_url(mut self, redirect_url: String) -> Self {
        self.oauthConfig = self.oauthConfig.set_redirect_url::<&str>(&redirect_url);
        self
    }
    pub fn authenticate(mut self) -> Result<(), Box<std::error::Error>> {
        // Set the state parameter (optional)
        self.oauthConfig = self
            .oauthConfig
            .set_state(rand::thread_rng().gen::<u16>().to_string());

        // Generate the authorization URL to which we'll redirect the user.
        let authorize_url = self.oauthConfig.authorize_url();

        if webbrowser::open(&authorize_url.to_string()).is_ok() {
            println!("{}", &authorize_url.to_string())
        }

        // // These variables will store the code & state retrieved during the authorization process.
        let mut code = String::new();

        // A very naive implementation of the redirect server.
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    {
                        let mut reader = BufReader::new(&stream);

                        let mut request_line = String::new();
                        reader.read_line(&mut request_line).unwrap();

                        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                        let url =
                            Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                        let code_pair = url
                            .query_pairs()
                            .find(|pair| {
                                let &(ref key, _) = pair;
                                key == "code"
                            })
                            .unwrap();

                        let (_, value) = code_pair;
                        code = value.into_owned();
                    }

                    let message = "Go back to your terminal :)";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    );
                    stream.write_all(response.as_bytes()).unwrap();

                    // The server will terminate itself after collecting the first code.
                    break;
                }
                Err(e) => println!("{}", e),
            }
        }

        // Exchange the code with a token.
        let token = self.oauthConfig.exchange_code(code);

        println!("Google returned the following token:\n{:?}\n", token);

        Ok(())
    }
}

#[cfg(test)]
mod test;
