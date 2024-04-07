use std::any::Any;
use bevy::hierarchy::Children;
use bevy::prelude::{BackgroundColor, BorderColor, Button, Changed, Color, Interaction, Query, Ref, Text, With};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(mut interaction_query:
                     Query<(&Interaction,
                            &mut BackgroundColor,
                            &mut BorderColor,
                            &Children, ), (Changed<Interaction>, With<Button>), >) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let mut red = color.0.r();
                let mut green = color.0.g();
                let mut blue = color.0.b();
                *color = BackgroundColor::from(Color::rgb(red + 5.0, green + 5.0, blue + 5.0));
            }
            Interaction::Hovered => {
                let mut red = color.0.r();
                let mut green = color.0.g();
                let mut blue = color.0.b();
                *color = BackgroundColor::from(Color::rgb(red + 25.0, green + 25.0, blue + 25.0));
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                let mut red = color.0.r();
                let mut green = color.0.g();
                let mut blue = color.0.b();
                *color = BackgroundColor::from(Color::rgb(red - 25.0, green - 25.0, blue - 25.0));
                // border_color.0 = Color::BLACK;
            }
        }
    }
}