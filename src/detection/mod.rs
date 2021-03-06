//! Collision detection and joints.

pub use detection::detector::Detector;
pub use detection::bodies_bodies::{BodiesBodies, BodyBodyDispatcher};
pub use detection::activation_manager::ActivationManager;

pub mod constraint;

mod detector;
mod bodies_bodies;

/// Joint handling.
pub mod joint {
    pub use detection::joint::anchor::Anchor;
    pub use detection::joint::joint::Joint;
    pub use detection::joint::ball_in_socket::BallInSocket;
    pub use detection::joint::fixed::Fixed;
    pub use detection::joint::joint_manager::JointManager;

    mod joint_manager;
    mod anchor;
    mod ball_in_socket;
    mod fixed;
    mod joint;
}

mod activation_manager;
