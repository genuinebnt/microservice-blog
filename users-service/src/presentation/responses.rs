use common::pagination::PaginatedResponse;

use crate::domain::entities::user::User;

pub type ListUserResponse = PaginatedResponse<User>;
