use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(Handle<AudioSource>);

impl CollisionSound {
    pub fn new(audio: Handle<AudioSource>) -> Self {
        Self(audio)
    }

    pub fn get_source(&self) -> Handle<AudioSource> {
        self.0.clone()
    }
}
