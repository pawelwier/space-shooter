use bevy::prelude::*;

pub fn key_pressed (
    keyboard_input: &Res<Input<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.pressed(keycode) }

pub fn key_just_pressed (
    keyboard_input: &Res<Input<KeyCode>>,
    keycode: KeyCode
) -> bool { keyboard_input.just_pressed(keycode) }