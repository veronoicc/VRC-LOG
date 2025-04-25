use crate::{
    provider::{Provider, Type},
    USER_AGENT,
};
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use reqwest::StatusCode;

const URL: &str = "https://paw-api.amelia.fun/update";

pub struct PAW {
    client: Client,
}

impl Default for PAW {
    fn default() -> Self {
        Self {
            client: Client::default()
        }
    }
}

impl Provider for PAW {
    fn check_avatar_id(&self, _avatar_id: &str) -> Result<bool> {
        bail!("Cache Only")
    }

    fn send_avatar_id(&self, avatar_id: &str) -> Result<bool> {
        let response = self
            .client
            .post(URL)
            .header("User-Agent", USER_AGENT)
            .query(&[("avatarId", avatar_id)])
            .send()?;

        let status = response.status();
        let text = response.text()?;
        debug!("[{}] {status} | {text}", Type::VRCDS);

        let unique = match status {
            StatusCode::OK => false,
            _ => bail!("[{}] {status} | {text}", Type::VRCDS),
        };

        Ok(unique)
    }
}
