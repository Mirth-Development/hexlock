use bevy::prelude::*;

#[derive(Resource)]
pub struct EffectsSpriteHandles {
    pub lightning_effect: Handle<Image>,
    pub rust_effect: Handle<Image>
}

