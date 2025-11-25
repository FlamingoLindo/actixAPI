use crate::models::ResponseStatus;
use crate::models::user::dto::get_users::{GetUsersResponse, PaginationMeta};
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn get_users_paginated(
        pool: &PgPool,
        username: Option<&str>,
        page: i64,
        limit: i64,
    ) -> Result<GetUsersResponse, sqlx::Error> {
        let offset = (page - 1) * limit;

        let (users, total) =
            UserRepository::get_users_with_count(pool, username, limit, offset).await?;

        let total_pages = if total == 0 {
            1
        } else {
            (total + limit - 1) / limit
        };

        let total_in_page = users.len();

        let response = GetUsersResponse {
            status: ResponseStatus::Success,
            users,
            pagination: PaginationMeta {
                total_in_page,
                total,
                total_pages,
                current_page: page,
                page_size: limit,
            },
        };

        Ok(response)
    }
}
