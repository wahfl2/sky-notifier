use poise::{serenity_prelude as serenity, CreateReply};

pub trait CreateReplyEx {
    fn button(self, button: serenity::CreateButton) -> Self;
    fn embed_replace(self, embed: serenity::CreateEmbed) -> Self;
    fn embeds_replace(self, embeds: Vec<serenity::CreateEmbed>) -> Self;
}

impl CreateReplyEx for CreateReply {
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