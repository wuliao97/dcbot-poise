use serenity::all::{ButtonStyle, CreateActionRow, CreateButton};

#[derive(Debug)]
pub struct EZButton {
    buttons: Vec<CreateButton>
}
impl EZButton {
    pub fn new() -> Self {
        Self {
            buttons: vec![]
        }
    }

    pub fn add_btn<T: Into<String>>(
        &mut self,
        custom_id: T,
        label: T,
        style: Option<ButtonStyle>
    ) -> &mut Self {
        let mut btn = CreateButton::new(custom_id.into())
            .label(label.into());

        if let Some(style) = style {
            btn = btn.style(style);
        }

        self.buttons.push(btn);

        self
    }

    pub fn build(mut self) -> Vec<CreateActionRow> {
        vec![
            CreateActionRow::Buttons(self.buttons)
        ]
    }
}