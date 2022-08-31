//! Allows using Rust functions in place of Bevy plugins without sacrificing the builder pattern

use bevy::prelude::*;

/// Implemented for `App` for the `fn_plugin` method
pub trait FnPluginExt {
    /// Runs `f` on `self`
    fn fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self;
}

impl FnPluginExt for App {
    fn fn_plugin(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}
