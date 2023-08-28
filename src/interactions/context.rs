use std::{marker::PhantomData, ops::Deref};

use thiserror::Error;
use twilight_model::{
    application::interaction::{
        application_command::{CommandData, CommandDataOption, CommandOptionValue},
        message_component::MessageComponentInteractionData,
        modal::ModalInteractionData,
        Interaction, InteractionData,
    },
    id::{marker::UserMarker, Id},
};

pub struct InteractionContext<T>(Interaction, PhantomData<T>);

impl<T> From<Interaction> for InteractionContext<T> {
    fn from(value: Interaction) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Deref for InteractionContext<T> {
    type Target = Interaction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type ApplicationCommandInteraction = InteractionContext<CommandData>;
pub type MessageComponentInteraction = InteractionContext<MessageComponentInteractionData>;
pub type ModalSubmitInteraction = InteractionContext<ModalInteractionData>;

impl ApplicationCommandInteraction {
    pub fn data(&self) -> &CommandData {
        match self.0.data.as_ref().unwrap() {
            InteractionData::ApplicationCommand(data) => data,
            _ => unreachable!(),
        }
    }
}

impl MessageComponentInteraction {
    pub fn data(&self) -> &MessageComponentInteractionData {
        match self.0.data.as_ref().unwrap() {
            InteractionData::MessageComponent(data) => data,
            _ => unreachable!(),
        }
    }
}

impl ModalSubmitInteraction {
    pub fn data(&self) -> &ModalInteractionData {
        match self.0.data.as_ref().unwrap() {
            InteractionData::ModalSubmit(data) => data,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandOptionError {
    #[error("option {0} not found")]
    NotFound(String),
    #[error("option {0} should be of type {1}")]
    WrongType(String, &'static str),
}

pub trait GetOption<T> {
    fn get_option<'a>(&'a self, name: &str) -> Result<&'a T, CommandOptionError>;
}

impl GetOption<String> for Vec<CommandDataOption> {
    fn get_option<'a>(&'a self, name: &str) -> Result<&'a String, CommandOptionError> {
        match self.iter().find(|option| option.name == name) {
            Some(option) => match &option.value {
                CommandOptionValue::String(value) => Ok(value),
                _ => Err(CommandOptionError::WrongType(name.to_string(), "String")),
            },
            None => Err(CommandOptionError::NotFound(name.to_string())),
        }
    }
}

impl GetOption<Id<UserMarker>> for Vec<CommandDataOption> {
    fn get_option<'a>(&'a self, name: &str) -> Result<&'a Id<UserMarker>, CommandOptionError> {
        match self.iter().find(|option| option.name == name) {
            Some(option) => match &option.value {
                CommandOptionValue::User(value) => Ok(value),
                _ => Err(CommandOptionError::WrongType(name.to_string(), "String")),
            },
            None => Err(CommandOptionError::NotFound(name.to_string())),
        }
    }
}

pub trait GetSubcommand {
    fn get_subcommand<'a>(
        &'a self,
    ) -> Result<(&'a str, &'a Vec<CommandDataOption>), CommandOptionError>;
    fn get_subcommand_group<'a>(
        &'a self,
    ) -> Result<(&'a str, &'a Vec<CommandDataOption>), CommandOptionError>;
}

impl GetSubcommand for Vec<CommandDataOption> {
    fn get_subcommand<'a>(
        &'a self,
    ) -> Result<(&'a str, &'a Vec<CommandDataOption>), CommandOptionError> {
        match self.get(0) {
            Some(option) => match &option.value {
                CommandOptionValue::SubCommand(value) => Ok((&option.name, value)),
                _ => Err(CommandOptionError::WrongType(
                    "subcommand".to_string(),
                    "SubCommand",
                )),
            },
            None => Err(CommandOptionError::NotFound("subcommand".to_string())),
        }
    }

    fn get_subcommand_group<'a>(
        &'a self,
    ) -> Result<(&'a str, &'a Vec<CommandDataOption>), CommandOptionError> {
        match self.get(0) {
            Some(option) => match &option.value {
                CommandOptionValue::SubCommandGroup(value) => Ok((&option.name, value)),
                _ => Err(CommandOptionError::WrongType(
                    "subcommand group".to_string(),
                    "SubCommandGroup",
                )),
            },
            None => Err(CommandOptionError::NotFound("subcommand group".to_string())),
        }
    }
}
