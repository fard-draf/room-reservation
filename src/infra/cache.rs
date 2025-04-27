use crate::error::{ErrRepo, ErrService};

use crate::features::{
    book::service::BookService, room::service::RoomService, user::service::UserService,
};

use super::db::DBClient;

pub async fn try_init_caches(
    user_service: &UserService<DBClient>,
    room_service: &RoomService<DBClient>,
    book_service: &BookService<DBClient>,
) -> Result<(), ErrService> {
    user_service
        .populate_cache()
        .await
        .map_err(|_| ErrService::Repo(ErrRepo::Unreachable))?;
    room_service
        .populate_cache()
        .await
        .map_err(|_| ErrService::Repo(ErrRepo::Unreachable))?;
    book_service
        .populate_cache()
        .await
        .map_err(|_| ErrService::Repo(ErrRepo::Unreachable))?;

    Ok(())
}
