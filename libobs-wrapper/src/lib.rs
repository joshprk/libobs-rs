use getters0::Getters;

#[cfg(not(windows))]
compiler_error!("libobs-wrapper can only be used in windows");

pub mod context;
pub mod crash_handler;
pub mod data;
pub mod display;
pub mod encoders;
pub mod logger;
pub mod runtime;
pub mod scenes;
pub mod signals;
pub mod sources;
pub mod unsafe_send;

pub mod enums;
pub mod utils;

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
        Self {
            x: inner.x,
            y: inner.y,
        }
    }
}

impl From<Vec2> for libobs::vec2 {
    fn from(val: Vec2) -> Self {
        libobs::vec2 {
            __bindgen_anon_1: libobs::vec2__bindgen_ty_1 {
                __bindgen_anon_1: libobs::vec2__bindgen_ty_1__bindgen_ty_1 { x: val.x, y: val.y },
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
