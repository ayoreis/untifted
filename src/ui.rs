use bevy::prelude::*;

pub const BLUE: Color = Color::hsv(234.0, 0.74, 0.91);
pub const LIGHT_BLUE: Color = Color::hsv(234.0, 0.54, 0.96);
pub const RED: Color = Color::hsv(4.0, 0.72, 0.89);
pub const LIGHT_RED: Color = Color::hsv(4.0, 0.52, 0.94);

pub fn button(marker: impl Component, background_color: Color) -> impl Bundle {
    (
        marker,
        Button,
        Node {
            padding: UiRect::axes(Val::Px(32.0), Val::Px(16.0)),
            ..default()
        },
        BackgroundColor(background_color),
        BorderRadius::all(Val::Px(8.0)),
    )
}
