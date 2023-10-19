use crate::api::options::Options;
use crate::api::Rest;
use dotenv::dotenv;
use std::future::Future;

pub fn test_with_credentials<C, Fut>(ctx: C) -> impl Future<Output = ()>
where
    C: FnOnce(Rest) -> Fut,
    Fut: Future<Output = ()>,
{
    dotenv().ok().expect("Failed to read .env file");

    async move {
        let api_key = std::env::var("OKX_API_KEY").expect("OKX_API_KEY not set");
        let api_secret = std::env::var("OKX_API_SECRET").expect("OKX_API_SECRET not set");
        let api_passphrase =
            std::env::var("OKX_API_PASSPHRASE").expect("OKX_API_PASSPHRASE not set");
        ctx(Rest::new(Options {
            key: Some(api_key),
            secret: Some(api_secret),
            passphrase: Some(api_passphrase),
        }))
        .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::v5::funding_account::GetCurrencies;

    #[tokio::test]
    async fn test_read_env_creds() {
        test_with_credentials(|rest| async move {
            let rval = rest.request(GetCurrencies::default()).await.unwrap();
            assert!(!rval.is_empty())
        })
        .await;
    }
}
