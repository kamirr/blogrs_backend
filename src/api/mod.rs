pub mod set_login_param;
pub mod manage_posts;
pub mod logout;
pub mod login;
pub mod meta;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        set_login_param::set_password,
        set_login_param::set_login,
        manage_posts::update,
        manage_posts::delete,
        manage_posts::new,
        manage_posts::get,
        logout::logout,
        login::login,
        meta::meta
    ]
}
