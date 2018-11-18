pub mod manage_posts;
pub mod logout;
pub mod login;
pub mod meta;
pub mod set;

mod webpost;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        manage_posts::update,
        manage_posts::delete,
        manage_posts::new,
        manage_posts::get,

        set::password_hash,
        set::login,

        logout::logout,
        login::login,

        meta::meta
    ]
}
