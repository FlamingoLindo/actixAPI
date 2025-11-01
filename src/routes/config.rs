use actix_web::web;

use super::user::{create_user, delete_user, get_user, get_users, update_user};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users")
        .service(get_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user);

    conf.service(scope);
}
