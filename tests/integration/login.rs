use crate::integration::register_test;

#[test]
fn command_login_help() {
    let _t = register_test("login/login-help.trycmd");
}

#[test]
fn command_login() {
    let _t = register_test("login/login.trycmd");
}
