use server_core::web::{res::Res, validator::ValidatedForm};
use server_model::admin::input::sys_authentication::LoginInput;

pub async fn login_handler(ValidatedForm(input): ValidatedForm<LoginInput>) -> Res<String> {
    // TODO: Verify username and password with database
    if input.account == "admin" && input.password == "password" {
        Res::new_data("login success".into())
    } else {
        Res::new_error(400, "login failed".into())
    }
}
