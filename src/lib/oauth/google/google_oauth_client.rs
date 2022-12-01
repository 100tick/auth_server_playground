use super::{GoogleToken, GoogleUserInfo};

const GOOGLE_ACCOUNTS_ENDPOINT: &str = "https://accounts.google.com";
const GOOGLE_OAUTH2_APIS_ENDPOINT: &str = "https://oauth2.googleapis.com";
const GOOGLE_APIS_ENDPOINT: &str = "https://googleapis.com";

#[derive(Clone)]
pub struct GoogleOAuthClient {
    code_url_for_login: String,
    code_url_for_create_user: String,
    token_url_for_login: String,
    token_url_for_create_user: String,
    user_info_url: String,
}

impl GoogleOAuthClient {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        login_redirect_url: &str,
        create_user_redirect_url: &str,
    ) -> Self {
        // let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        // let client_secret =
        //     env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");

        let code_url_for_login = code_url(GOOGLE_ACCOUNTS_ENDPOINT, client_id, login_redirect_url);
        let code_url_for_create_user = code_url(
            GOOGLE_ACCOUNTS_ENDPOINT,
            client_id,
            create_user_redirect_url,
        );

        let token_url_for_login = token_url(
            GOOGLE_OAUTH2_APIS_ENDPOINT,
            client_id,
            client_secret,
            login_redirect_url,
        );
        let token_url_for_create_user = token_url(
            GOOGLE_OAUTH2_APIS_ENDPOINT,
            client_id,
            client_secret,
            create_user_redirect_url,
        );

        let mut user_info_url = String::from(GOOGLE_APIS_ENDPOINT);
        user_info_url.push_str("/oauth2/v1/userinfo");

        Self {
            code_url_for_login,
            code_url_for_create_user,
            token_url_for_login,
            token_url_for_create_user,
            user_info_url,
        }
    }

    pub fn code_url_for_login(&self) -> String {
        self.code_url_for_login.to_string()
    }

    pub fn code_url_for_create_user(&self) -> String {
        self.code_url_for_create_user.to_string()
    }

    pub async fn get_token_for_login(&self, code: &str) -> Result<GoogleToken, reqwest::Error> {
        let mut url = self.token_url_for_login.clone();
        url.push_str(code);

        self.get_token(&url).await
    }

    pub async fn get_token_for_create_user(
        &self,
        code: &str,
    ) -> Result<GoogleToken, reqwest::Error> {
        let mut url = self.token_url_for_create_user.clone();
        url.push_str(code);

        self.get_token(&url).await
    }

    async fn get_token(&self, url: &str) -> Result<GoogleToken, reqwest::Error> {
        let resp = reqwest::Client::new().post(url).send().await?;

        let token: GoogleToken = resp.json().await?;
        Ok(token)
    }

    pub async fn get_user_info(
        &self,
        access_token: &str,
    ) -> Result<GoogleUserInfo, reqwest::Error> {
        let client = reqwest::Client::default();
        let resp = client
            .get(self.user_info_url.to_string())
            .header("Authorization", "Bearer ".to_string() + access_token)
            .send()
            .await?;

        let user_info: GoogleUserInfo = resp.json().await?;
        Ok(user_info)
    }
}

fn code_url(endpoint: &str, client_id: &str, redirect_uri: &str) -> String {
    let mut url = endpoint.to_string();

    url.push_str("/o/oauth2/v2/auth");
    url.push_str("?scope=email,profile");
    url.push_str("&access_type=offline");
    url.push_str("&include_granted_scopes=true");
    url.push_str("&response_type=code");
    url.push_str("&client_id=");
    url.push_str(client_id);
    url.push_str("&redirect_uri=");
    url.push_str(redirect_uri);

    url
}

fn token_url(endpoint: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> String {
    let mut url = endpoint.to_string();

    url.push_str("/token");
    url.push_str("?grant_type=authorization_code");
    url.push_str("&redirect_uri=");
    url.push_str(redirect_uri);
    url.push_str("&client_id=");
    url.push_str(client_id);
    url.push_str("&client_secret=");
    url.push_str(client_secret);
    url.push_str("&code=");

    url
}
