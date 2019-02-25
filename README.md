# oauth2-noserver-rs
Handles the oauth2 flow for installed apps that don't have a server side.

Oauth was designed for letting third party servers access your data. Which makes it a bit overcomplicated for when you simply want your users to access their data through software that runs only on their computer.
This project aims to simplify that a lot for Rust projects, as a library.

# Quick start

Once you have an oauth2 config...
```rust
let oauthConfig = oauth2::Config::new(
    "someID",
    "someSECRET",
    "https://accounts.google.com/o/oauth2/v2/auth",
    "https://www.googleapis.com/oauth2/v3/token",
)
.add_scope("https://www.googleapis.com/auth/calendar")
.add_scope("https://www.googleapis.com/auth/plus.me");
```

...use this to open the browser, ask your user to connect to the service, and get a token back to your program to act on behalf of the user:
```rust
let authenticator = oauth2_noserver::Authenticator::new(oauthConfig);
authenticator.authenticate().unwrap();
```

## Installing
Add this to your Cargo.toml:`oauth2_noserver = "0.1.0"`

## Prerequisites

- Rust 1.32+
- **Register an oauth app on the service you want**
    - Configure it with your Authenticator's redirection URL: by default, **`http://localhost:14565/oauth/callback`**, or replace with what you specified in `Authenticator.set_redirect_url()`
    - get credentials once it's created (client_id and client_secret). These are the ones you need to be able to create an oauth2::Config

## Configuration

- `Authenticator.set_port(port)`: the lib will spin up a temporary server to await the token from the service on this port.
    Use the same port in your redirect_url, both in the Authenticator config and when you create your Oauth credentials on 
    Optional. 14565 by default.
- `Authenticator.set_redirect_url(Into<String>)`: set it to the same redirect_url you used when creating your oauth app on the service
    Optional. By default, `http://localhost:14565/oauth/callback`.


# Examples

## Minimal example
```rust
fn main() -> Result<(), Box<std::error::Error>> {
    let oauthConfig = Oauth2Config::new(
        "someID",
        "someSECRET",
        "https://accounts.google.com/o/oauth2/v2/auth",
        "https://www.googleapis.com/oauth2/v3/token",
    )
    .add_scope("https://www.googleapis.com/auth/calendar")
    .add_scope("https://www.googleapis.com/auth/plus.me");

    let authenticator = oauth2_noserver::Authenticator::new(oauthConfig);
    authenticator.authenticate().unwrap();

    Ok(())
}
```

## Fully configured
```rust
fn main() -> Result<(), Box<std::error::Error>> {
    let oauthConfig = Oauth2Config::new(
        "someID",
        "someSECRET",
        "https://accounts.google.com/o/oauth2/v2/auth",
        "https://www.googleapis.com/oauth2/v3/token",
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
