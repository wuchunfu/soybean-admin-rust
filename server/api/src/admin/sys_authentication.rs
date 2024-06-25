use server_core::web::{res::Res, validator::ValidatedForm};
use server_service::admin::LoginInput;

pub struct SysAuthenticationApi;

impl SysAuthenticationApi {
    pub async fn login_handler(ValidatedForm(input): ValidatedForm<LoginInput>) -> Res<String> {
        // TODO: Verify username and password with database
        if input.account == "admin" && input.password == "password" {
            Res::new_data("login success".into())
        } else {
            Res::new_error(400, "login failed".into())
        }
    }
}
