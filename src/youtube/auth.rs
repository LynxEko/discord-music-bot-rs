use std::sync::OnceLock;

use crate::config::Config;

use google_youtube3::{
    hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector,
    yup_oauth2::authenticator::Authenticator,
};

static GLOBAL_YOUTUBE: OnceLock<YouTubeAuth> = OnceLock::new();

pub struct YouTubeAuth {
    pub auth: Authenticator<HttpsConnector<HttpConnector>>,
}

impl YouTubeAuth {
    async fn new() -> Self {
        let config = Config::get();

        // let auth =
        //     google_youtube3::oauth2::ServiceAccountAuthenticator::builder(ServiceAccountKey {
        //         key_type: Some(config.youtube_auth_type.clone()),
        //         project_id: Some(config.youtube_project_id.clone()),
        //         private_key_id: Some(config.youtube_private_key_id.clone()),
        //         private_key: config.youtube_private_key.clone(),
        //         client_email: config.youtube_client_email.clone(),
        //         client_id: Some(config.youtube_client_id.clone()),
        //         auth_uri: Some(config.youtube_auth_uri.clone()),
        //         token_uri: config.youtube_token_uri.clone(),
        //         auth_provider_x509_cert_url: Some(
        //             config.youtube_auth_provider_x509_cert_url.clone(),
        //         ),
        //         client_x509_cert_url: Some(config.youtube_client_x509_cert_url.clone()),
        //     })
        //     .build()
        //     .await
        //     .unwrap();

        let auth = google_youtube3::yup_oauth2::ServiceAccountAuthenticator::builder(
            google_youtube3::yup_oauth2::ServiceAccountKey {
                key_type: Some(config.youtube_auth_type.clone()),
                private_key_id: Some(config.youtube_private_key_id.clone()),
                private_key: config.youtube_private_key.clone(),
                client_id: Some(config.youtube_client_id.clone()),
                token_uri: config.youtube_token_uri.clone(),
                auth_uri: Some(config.youtube_auth_uri.clone()),
                project_id: Some(config.youtube_project_id.clone()),
                client_email: config.youtube_client_email.clone(),
                auth_provider_x509_cert_url: Some(
                    config.youtube_auth_provider_x509_cert_url.clone(),
                ),
                client_x509_cert_url: Some(config.youtube_client_x509_cert_url.clone()),
            },
        )
        .build()
        .await
        .unwrap();

        Self { auth }
    }

    pub async fn init() {
        let new_val = Self::new().await;
        GLOBAL_YOUTUBE.get_or_init(|| new_val);
    }

    pub fn get() -> &'static Self {
        GLOBAL_YOUTUBE.get().unwrap()
    }
}
