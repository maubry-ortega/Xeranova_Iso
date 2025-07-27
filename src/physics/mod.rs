//! Sistema de físicas personalizado sin dependencias externas

pub mod velocity;
pub mod collision;

pub use velocity::Velocity;
pub use collision::apply_velocity_and_collisions;