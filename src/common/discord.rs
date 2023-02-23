use serenity::builder::CreateButton;
use serenity::model::application::component::ButtonStyle;

pub fn text_button(name: &str) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(name);
    b.label(name);
    b.style(ButtonStyle::Primary);
    b
}
