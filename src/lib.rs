#![deny(missing_docs)]

//! pavlok
//!
use serde;
use reqwest;
use std::fmt;

/// Error
pub struct Error {
   kind: Kind, 
   inner: Option<reqwest::Error>,
}

impl Error {
    pub(crate) fn new(kind:Kind, inner: Option<reqwest::Error>) -> Error {
        Error {
            kind: kind,
            inner: inner,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::OutofBounds => f.write_str("strength is out of bounds")?,
			Kind::Reqwest => {
				match &self.inner {
					None => f.write_str("Unknown error")?,
					Some(err) => err.fmt(f)?,
				};
			},
        };

        Ok(())
    }
}


/// Kind is the kind of error that happened
#[derive(Debug)]
enum Kind {
    OutofBounds,
    Reqwest,
}

/// A `Result` alias where the `Err` case is `pavlok::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// StimuliResponse
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StimuliResponse {
    success: bool,
    id: String,
}

/// blocking version of the client
pub mod blocking {
    use super::*;
    use reqwest;
    /// Blocking Pavlok Client
    pub struct Client {
        access_token: String,
        client: reqwest::blocking::Client,
    }

    impl Client {
        /// Create a new Blocking Pavlok Client
        pub fn new(access_token: String) -> Client{
            Client {
                access_token: access_token,
                client: reqwest::blocking::Client::new(),
            }
        }

        /// Send a shock command to a Pavlok
        pub fn shock(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
            self.send("shock", strength, reason)
        }

        /// Send a beep command to a Pavlok
        pub fn beep(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
            self.send("beep", strength, reason)
        }

        /// Send a vibration command to a Pavlok
        pub fn vibrate(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
            self.send("vibration", strength, reason)
        }

        /// Send an led command to a Pavlok
        pub fn led(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
            self.send("led", strength, reason)
        }

        fn send(&self, name: &str, strength: u8, reason: &str) -> Result<StimuliResponse> {
            Ok(self.client.post(self.url(name, strength))
               .query(&[ ("access_token", &self.access_token), ("reason", &reason.to_string())])
               .send().map_err(|err| Error::new(Kind::Reqwest, Some(err)))?
               .json::<StimuliResponse>().map_err(|err| Error::new(Kind::Reqwest, Some(err)))?)
        }

        fn url(&self, stimili: &str, strength: u8) -> String {
            format!("https://app.pavlok.com/api/v1/{}/{}", stimili, strength)
        }
    }
}

/// An async Client
pub struct Client {
    access_token: String,
    client: reqwest::Client,
}

impl Client {
        
    /// Create an new async client
    pub fn new(access_token: String) -> Client{
        Client {
            access_token: access_token,
            client: reqwest::Client::new(),
        }
    }

    /// Send a shock command to a Pavlok
    pub async fn shock(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
        if strength == 0 {
            return Err(Error::new(Kind::OutofBounds, None));
        }

        self.send("shock", strength, reason).await
    }

    /// Send a beep command to a Pavlok
    pub async fn beep(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
        if strength == 0 || strength > 4 {
            return Err(Error::new(Kind::OutofBounds, None));
        }

        self.send("beep", strength, reason).await
    }

    /// Send a vibration command to a Pavlok
    pub async fn vibrate(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
        if strength == 0 {
            return Err(Error::new(Kind::OutofBounds, None));
        }

        self.send("vibration", strength, reason).await
    }

    /// Send an led command to a Pavlok
    pub async fn led(&self, strength: u8, reason: &str) -> Result<StimuliResponse> {
        if strength == 0 || strength > 4 {
            return Err(Error::new(Kind::OutofBounds, None));
        }
        self.send("led", strength, reason).await
    }

    async fn send(&self, name: &str, strength: u8, reason: &str) -> Result<StimuliResponse> {
        Ok(self.client.post(self.url(name, strength))
           .query(&[ ("access_token", &self.access_token), ("reason", &reason.to_string())])
           .send().await.map_err(|err| Error::new(Kind::Reqwest, Some(err)))?
           .json::<StimuliResponse>().await.map_err(|err| Error::new(Kind::Reqwest, Some(err)))?)
    }

    fn url(&self, stimili: &str, strength: u8) -> String {
        format!("https://app.pavlok.com/api/v1/{}/{}", stimili, strength)
    }
}
