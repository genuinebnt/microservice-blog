use crate::domain::entities::notification::Notification;
use common::pagination::PaginatedResponse;

pub type ListNotificationResponse = PaginatedResponse<Notification>;
