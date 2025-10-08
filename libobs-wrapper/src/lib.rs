use getters0::Getters;

pub use libobs as sys;

#[cfg(not(windows))]
compiler_error!("libobs-wrapper can only be used in windows");

pub mod unsafe_send;
pub mod crash_handler;
pub mod data;
pub mod sources;
pub mod encoders;
pub mod context;
pub mod logger;
pub mod signals;
pub mod display;
pub mod scenes;
#[cfg(feature="bootstrapper")]
pub mod bootstrap;
pub mod runtime;

pub mod utils;
pub mod enums;

// Add the macros module to the public exports
mod macros;

#[derive(Debug, Clone, Copy, Getters)]
pub struct Vec2 {
    #[get_mut]
    x: f32,
    #[get_mut]
    y: f32,
}

impl From<libobs::vec2> for Vec2 {
    fn from(raw: libobs::vec2) -> Self {
        let inner = unsafe { raw.__bindgen_anon_1.__bindgen_anon_1 };
        Self { x: inner.x, y: inner.y }
    }
}

impl Into<libobs::vec2> for Vec2 {
    fn into(self) -> libobs::vec2 {
        libobs::vec2 {
            __bindgen_anon_1: libobs::vec2__bindgen_ty_1 {
                __bindgen_anon_1: libobs::vec2__bindgen_ty_1__bindgen_ty_1 { x: self.x, y: self.y },
            },
        }
    }
}

#[test]
fn test_vec2() {
    let vec_val = Vec2::new(1.0, 2.0);
    let libobs_vec: libobs::vec2 = vec_val.into();
    
    let original = Vec2::from(libobs_vec);
    assert_eq!(original.x, 1.0);
    assert_eq!(original.y, 2.0);
    assert_ne!(original.x, 0.0);
    assert_ne!(original.y, 0.0);
}