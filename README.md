# spacex_sdk

## Example

run as `cargo run --example simple`

auto generate:

```rust
use jwfetch::{FetchError, Method};
use log::info;
use serde::{Deserialize, Serialize};
use spacex_sdk::{get_auto_open_api, GetAccessTokenOptions, GetOpenAPIOptions};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct UserInfo {
    username: String,
    email: String,
}

pub async fn get_user_info() -> Result<UserInfo, FetchError> {
    let params = GetAccessTokenOptions {
        client_id: "xx".to_string(),
        client_secret: "xx".to_string(),
        scope: "wecom".to_string(),
        user_agent: "xxx".to_string(),
        auth_api: "https://open.xx.com/sso/v2".to_string(),
    };
    let openid = "xx".to_string();
    let url = format!("account/get_userinfo_by_openid?openid={}", openid);
    let open_params = GetOpenAPIOptions {
        client_id: params.client_id.clone(),
        user_agent: params.user_agent.clone(),
        url,
        method: Method::GET,
        authorization_type: "Basic".to_string(),
        body: None,
        auth_api: params.auth_api.clone(),
        timeout: None,
    };
    get_auto_open_api::<UserInfo>(params, open_params).await
}

#[tokio::main]
async fn main() {
    match get_user_info().await {
        Ok(user) => {
            info!("userinfo: {:?}", user);
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
}
```