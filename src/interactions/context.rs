use std::{marker::PhantomData, ops::Deref};

use twilight_model::application::interaction::{
    application_command::CommandData, message_component::MessageComponentInteractionData,
    modal::ModalInteractionData, Interaction, InteractionData,
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
