use poise::serenity_prelude as serenity;
use serenity::all::{CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption};

pub struct EZSelect {
    custom_id: String,
    options: Vec<String>,
    placeholder: Option<String>,
    max_value: Option<u8>,
    min_value: Option<u8>,
}
impl EZSelect {

    #[inline]
    pub fn new(options: Vec<String>, custom_id: impl ToString, place_holder: Option<&str>, max_value: Option<u8>, min_value: Option<u8>) -> Self
    {
        let custom_id = custom_id.to_string();
        let mut placeholder= None;
        if let Some (holder) = place_holder {
            placeholder = Some(holder.to_string());
        }

        Self {
            custom_id,
            options,
            placeholder,
            max_value,
            min_value,
        }
    }

    pub fn build(&self) -> Vec<CreateActionRow> {
        let options: Vec<CreateSelectMenuOption> = self.options.iter().enumerate()
            .map(|(idx, column)| CreateSelectMenuOption::new(column, idx.to_string()))
            .collect();
        vec![CreateActionRow::SelectMenu(CreateSelectMenu::new(
            &self.custom_id, CreateSelectMenuKind::String { options })
            .min_values(self.max_value.unwrap_or(1))
            .max_values(self.min_value.unwrap_or(1))
            // .placeholder(self.placeholder.unwrap_or("Choose your want a Song!".to_string()))
        )]
    }
}