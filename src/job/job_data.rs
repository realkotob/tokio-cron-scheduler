#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJob {
    #[prost(string, tag="1")]
    pub schedule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonCronJob {
    #[prost(bool, tag="1")]
    pub repeating: bool,
    #[prost(uint64, tag="2")]
    pub repeated_every: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(uint64, tag="1")]
    pub id1: u64,
    #[prost(uint64, tag="2")]
    pub id2: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStoredData {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<Uuid>,
    #[prost(uint64, optional, tag="2")]
    pub last_updated: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub last_tick: ::core::option::Option<u64>,
    #[prost(uint64, tag="4")]
    pub next_tick: u64,
    #[prost(enumeration="JobType", tag="5")]
    pub job_type: i32,
    #[prost(uint32, tag="8")]
    pub count: u32,
    #[prost(bytes="vec", tag="9")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="10")]
    pub ran: bool,
    #[prost(bool, tag="11")]
    pub stopped: bool,
    #[prost(oneof="job_stored_data::Job", tags="6, 7")]
    pub job: ::core::option::Option<job_stored_data::Job>,
}
/// Nested message and enum types in `JobStoredData`.
pub mod job_stored_data {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        #[prost(message, tag="6")]
        CronJob(super::CronJob),
        #[prost(message, tag="7")]
        NonCronJob(super::NonCronJob),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobIdAndNotification {
    #[prost(message, optional, tag="1")]
    pub job_id: ::core::option::Option<Uuid>,
    #[prost(message, optional, tag="2")]
    pub notification_id: ::core::option::Option<Uuid>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationData {
    #[prost(message, optional, tag="1")]
    pub job_id: ::core::option::Option<JobIdAndNotification>,
    #[prost(enumeration="JobState", repeated, tag="2")]
    pub job_states: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes="vec", tag="3")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationIdAndState {
    #[prost(message, optional, tag="1")]
    pub notification_id: ::core::option::Option<Uuid>,
    #[prost(enumeration="JobState", tag="2")]
    pub job_state: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobAndNextTick {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<Uuid>,
    #[prost(enumeration="JobType", tag="2")]
    pub job_type: i32,
    #[prost(uint64, tag="3")]
    pub next_tick: u64,
    #[prost(uint64, optional, tag="4")]
    pub last_tick: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfUuids {
    #[prost(message, repeated, tag="1")]
    pub uuids: ::prost::alloc::vec::Vec<Uuid>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobAndNotifications {
    #[prost(message, optional, tag="1")]
    pub job_id: ::core::option::Option<Uuid>,
    #[prost(message, repeated, tag="2")]
    pub notification_ids: ::prost::alloc::vec::Vec<Uuid>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfJobsAndNotifications {
    #[prost(message, repeated, tag="1")]
    pub job_and_notifications: ::prost::alloc::vec::Vec<JobAndNotifications>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    Stop = 0,
    Scheduled = 1,
    Started = 2,
    Done = 3,
    Removed = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobType {
    Cron = 0,
    Repeated = 1,
    OneShot = 2,
}