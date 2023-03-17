use crate::domain::{SubscriberName, SubscriberEmail};

#[derive(Debug)]
pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}
