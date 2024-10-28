use libobs::{
    gs_color_format_GS_A8, gs_color_format_GS_BGRA, gs_color_format_GS_BGRA_UNORM,
    gs_color_format_GS_BGRX, gs_color_format_GS_BGRX_UNORM, gs_color_format_GS_DXT1,
    gs_color_format_GS_DXT3, gs_color_format_GS_DXT5, gs_color_format_GS_R10G10B10A2,
    gs_color_format_GS_R16, gs_color_format_GS_R16F, gs_color_format_GS_R32F,
    gs_color_format_GS_R8, gs_color_format_GS_R8G8, gs_color_format_GS_RG16,
    gs_color_format_GS_RG16F, gs_color_format_GS_RG32F, gs_color_format_GS_RGBA,
    gs_color_format_GS_RGBA16, gs_color_format_GS_RGBA16F, gs_color_format_GS_RGBA32F,
    gs_color_format_GS_RGBA_UNORM, gs_color_format_GS_UNKNOWN, gs_zstencil_format_GS_Z16,
    gs_zstencil_format_GS_Z24_S8, gs_zstencil_format_GS_Z32F, gs_zstencil_format_GS_Z32F_S8X24,
    gs_zstencil_format_GS_ZS_NONE,
};
use num_derive::{FromPrimitive, ToPrimitive};

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum GsColorFormat {
    Unknown = gs_color_format_GS_UNKNOWN,
    A8 = gs_color_format_GS_A8,
    R8 = gs_color_format_GS_R8,
    RGBA = gs_color_format_GS_RGBA,
    BGRX = gs_color_format_GS_BGRX,
    BGRA = gs_color_format_GS_BGRA,
    R10G10B10A2 = gs_color_format_GS_R10G10B10A2,
    RGBA16 = gs_color_format_GS_RGBA16,
    R16 = gs_color_format_GS_R16,
    RGBA16F = gs_color_format_GS_RGBA16F,
    RGBA32F = gs_color_format_GS_RGBA32F,
    RG16F = gs_color_format_GS_RG16F,
    RG32F = gs_color_format_GS_RG32F,
    R16F = gs_color_format_GS_R16F,
    R32F = gs_color_format_GS_R32F,
    DXT1 = gs_color_format_GS_DXT1,
    DXT3 = gs_color_format_GS_DXT3,
    DXT5 = gs_color_format_GS_DXT5,
    R8G8 = gs_color_format_GS_R8G8,
    RGBAUnorm = gs_color_format_GS_RGBA_UNORM,
    BGRXUnorm = gs_color_format_GS_BGRX_UNORM,
    BGRAUnorm = gs_color_format_GS_BGRA_UNORM,
    RG16 = gs_color_format_GS_RG16,
}

#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum GsZstencilFormat {
    ZSNone = gs_zstencil_format_GS_ZS_NONE,
    Z16 = gs_zstencil_format_GS_Z16,
    Z24s8 = gs_zstencil_format_GS_Z24_S8,
    Z32F = gs_zstencil_format_GS_Z32F,
    Z32s8X24 = gs_zstencil_format_GS_Z32F_S8X24,
}
