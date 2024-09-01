use crate::reminderhandler::NewReminder as NewReminderDto;
use crate::reminderhandler::Reminder as ReminderDto;
use crate::reminderhandler::Urgency as UrgencyDto;
use chrono::{DateTime, Utc};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use mongodb::bson::doc;
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Reminder {
    id: Uuid,
    active: bool,
    title: String,
    description: Option<String>,
    urgency: Urgency,
    started: DateTime<Utc>,
    deadline: DateTime<Utc>,
}

impl Reminder {
    pub fn new(
        title: String,
        description: Option<String>,
        urgency: Option<Urgency>,
        deadline: DateTime<Utc>,
    ) -> Self {
        Reminder {
            id: Uuid::new_v4(),
            active: true,
            title,
            description,
            started: Utc::now(),
            deadline,
            urgency: urgency.unwrap_or_default(),
        }
    }

    pub async fn insert(&self, collection: &mongodb::Collection<Reminder>) -> Result<()> {
        collection.insert_one(self).await?;
        Ok(())
    }

    pub async fn find_by_title(
        title: &str,
        collection: &mongodb::Collection<Reminder>,
    ) -> Result<Option<Reminder>> {
        let filter = doc! { "title:": title};
        let reminder = collection.find_one(filter).await?;
        Ok(reminder)
    }
}

impl TryFrom<NewReminderDto> for Reminder {
    type Error = color_eyre::eyre::Report;

    fn try_from(dto: NewReminderDto) -> Result<Self> {
        Ok(Reminder::new(
            dto.title.clone(),
            dto.description.clone(),
            Some(dto.urgency().into()),
            prost_timestamp_to_chrono(dto.deadline)
                .ok_or_else(|| eyre!("Failed to convert `started` timestamp"))?,
        ))
    }
}

impl TryFrom<ReminderDto> for Reminder {
    type Error = color_eyre::eyre::Report;

    fn try_from(dto: ReminderDto) -> Result<Self> {
        Ok(Reminder {
            id: Uuid::parse_str(&dto.id)?,
            active: dto.active,
            title: dto.title.clone(),
            description: dto.description.clone(),
            urgency: dto.urgency().into(),
            started: prost_timestamp_to_chrono(dto.started)
                .ok_or_else(|| eyre!("Failed to convert `started` timestamp"))?,
            deadline: prost_timestamp_to_chrono(dto.deadline)
                .ok_or_else(|| eyre!("Failed to convert `started` timestamp"))?,
        })
    }
}

impl From<Reminder> for ReminderDto {
    fn from(reminder: Reminder) -> Self {
        ReminderDto {
            id: reminder.id.into(),
            active: reminder.active,
            title: reminder.title.clone(),
            description: reminder.description.clone(),
            urgency: reminder.urgency.into(),
            started: Some(chrono_to_prost_timestamp(reminder.started)),
            deadline: Some(chrono_to_prost_timestamp(reminder.deadline)),
        }
    }
}

fn prost_timestamp_to_chrono(timestamp: Option<Timestamp>) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp?.seconds, timestamp?.nanos.try_into().ok()?)
}

fn chrono_to_prost_timestamp(datetime: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as i32,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Urgency {
    Extreme,
    High,
    Normal,
    Low,
}

impl Default for Urgency {
    fn default() -> Self {
        Urgency::Normal
    }
}

impl From<UrgencyDto> for Urgency {
    fn from(proto_urgency: UrgencyDto) -> Self {
        match proto_urgency {
            UrgencyDto::Extreme => Urgency::Extreme,
            UrgencyDto::High => Urgency::High,
            UrgencyDto::Normal => Urgency::Normal,
            UrgencyDto::Low => Urgency::Low,
        }
    }
}

impl From<Urgency> for UrgencyDto {
    fn from(urgency: Urgency) -> Self {
        match urgency {
            Urgency::Extreme => UrgencyDto::Extreme,
            Urgency::High => UrgencyDto::High,
            Urgency::Normal => UrgencyDto::Normal,
            Urgency::Low => UrgencyDto::Low,
        }
    }
}

impl From<Urgency> for i32 {
    fn from(urgency: Urgency) -> Self {
        match urgency {
            Urgency::Extreme => 0,
            Urgency::High => 1,
            Urgency::Normal => 2,
            Urgency::Low => 3,
        }
    }
}
