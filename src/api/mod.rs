pub mod manage_posts;
pub mod logout;
pub mod login;
pub mod meta;
pub mod set;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        manage_posts::get_json,
        manage_posts::update,
        manage_posts::delete,
        manage_posts::new,

        set::password_hash,
        set::login,

        logout::logout,
        login::login,

        meta::meta
    ]
}
