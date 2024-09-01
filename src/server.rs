use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Code, Request, Response, Status};

use crate::database::DbConnection;
use crate::reminder::Reminder;
use crate::reminderhandler as proto;
use crate::reminderhandler::reminder_handler_server::{ReminderHandler, ReminderHandlerServer};

#[derive(Debug)]
struct ReminderHandlerService {
    db: DbConnection<Reminder>,
}

#[tonic::async_trait]
impl ReminderHandler for ReminderHandlerService {
    async fn create_reminder(
        &self,
        request: Request<proto::CreateReminderRequest>,
    ) -> Result<Response<proto::CreateReminderResponse>, Status> {
        let new_reminder = match request.into_inner().reminder {
            Some(reminder) => reminder,
            None => return Err(Status::new(Code::Aborted, "No Data for creation")),
        };

        let reminder = match Reminder::try_from(new_reminder) {
            Ok(reminder) => reminder,
            Err(report) => return Err(Status::new(Code::Aborted, report.to_string())),
        };

        match reminder.insert(&self.db.collection).await {
            Ok(()) => (),
            Err(report) => return Err(Status::new(Code::Aborted, report.to_string())),
        }

        Ok(Response::new(proto::CreateReminderResponse {
            created: true,
            reminder: Some(reminder.into()),
        }))
    }

    type GetAllRemindersStream = ReceiverStream<Result<proto::GetAllRemindersResponse, Status>>;

    async fn get_all_reminders(
        &self,
        request: Request<proto::GetAllRemindersRequest>,
    ) -> Result<Response<Self::GetAllRemindersStream>, Status> {
        todo!("TODO")
    }

    async fn get_reminder_by_id(
        &self,
        request: Request<proto::GetReminderByIdRequest>,
    ) -> Result<Response<proto::GetReminderByIdResponse>, Status> {
        todo!("TODO")
    }

    type SearchReminderByTitleStream =
        ReceiverStream<Result<proto::SearchReminderByTitleResponse, Status>>;

    async fn search_reminder_by_title(
        &self,
        request: Request<proto::SearchReminderByTitleRequest>,
    ) -> Result<Response<Self::SearchReminderByTitleStream>, Status> {
        todo!("TODO")
    }
}
