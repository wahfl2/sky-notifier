use poise::{serenity_prelude::{self as serenity, CreateEmbed}, CreateReply};

use crate::types::{Context, McPlayer};

pub trait CreateReplyEx {
    fn simple_embed(description: impl Into<String>) -> Self;
    fn button(self, button: serenity::CreateButton) -> Self;
    fn embed_replace(self, embed: serenity::CreateEmbed) -> Self;
    fn embeds_replace(self, embeds: Vec<serenity::CreateEmbed>) -> Self;
}

impl CreateReplyEx for CreateReply {
    fn simple_embed(description: impl Into<String>) -> Self {
        Self::default().embed(CreateEmbed::new().description(description))
    }

    /// Adds a clickable button to this message.
    ///
    /// Convenience method that wraps [`Self::components`]. Arranges buttons in action rows
    /// automatically.
    /// 
    /// Copied from [`serenity::builder::button_and_select_menu_convenience_methods`]
    fn button(mut self, button: serenity::CreateButton) -> Self {
        let rows = self.components.get_or_insert_with(Vec::new);
        let row_with_space_left = rows.last_mut().and_then(|row| match row {
            serenity::CreateActionRow::Buttons(buttons) if buttons.len() < 5 => Some(buttons),
            _ => None,
        });

        match row_with_space_left {
            Some(row) => row.push(button),
            None => rows.push(serenity::CreateActionRow::Buttons(vec![button])),
        }

        self
    }
    
    fn embed_replace(mut self, embed: serenity::CreateEmbed) -> Self {
        self.embeds = vec![embed];
        self
    }
    
    fn embeds_replace(mut self, embeds: Vec<serenity::CreateEmbed>) -> Self {
        self.embeds = embeds;
        self
    }
}

pub trait ContextEx {
    async fn mc_player(&self) -> Option<McPlayer>;
}

impl ContextEx for Context<'_> {
    async fn mc_player(&self) -> Option<McPlayer> {
        let lock = self.data().discord_to_mc.lock().await;
        lock.get(&self.author().id.get()).cloned()
    }
}