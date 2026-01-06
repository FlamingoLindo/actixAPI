use super::admin_routes::create_admin::create_admin;
use super::user_routes::{
    create_user::create_user, delete_user::delete_user, get_user::get_user, get_users::get_users,
    update_user::update_user,
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use super::auth_routes::login::login;
use super::game_routes::create_game::create_game;
use crate::middleware::auth::validator;

pub fn config(conf: &mut web::ServiceConfig) {
    let auth_scope = web::scope("/api/auth").service(login);

    let auth_middleware = HttpAuthentication::bearer(validator);

    let users_scope = web::scope("/api/users")
        .service(create_user)
        .service(update_user)
        .service(get_user)
        .service(get_users)
        .service(
            web::scope("")
                .wrap(auth_middleware.clone())
                .service(delete_user),
        );

    let games_scope = web::scope("/api/games").service(create_game);

    let admin_scope = web::scope("/api/admins")
        .wrap(auth_middleware)
        .service(create_admin);

    conf.service(auth_scope);
    conf.service(users_scope);
    conf.service(admin_scope);
    conf.service(games_scope);
}
