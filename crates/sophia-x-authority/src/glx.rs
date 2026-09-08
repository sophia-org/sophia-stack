//! The GLX framebuffer configurations Sophia offers, and the bounds that go with
//! them.
//!
//! One owner for facts that were derived in two places and were about to be
//! derived in a third: the catalog answers `GetFBConfigs`, the runtime resolves a
//! drawable's depth from the same rows, and a pbuffer's refusal threshold is the
//! maximum this module advertises. Two copies of a fact are a drift waiting to
//! happen; three are one that already has.

/// One framebuffer configuration, as a passive row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XGlxFbConfig {
    pub id: u32,
    pub visual: u32,
    /// Alpha bits: zero for the opaque visual, eight for ARGB.
    pub alpha: u32,
    /// Whether the configuration advertises sRGB framebuffer capability.
    pub srgb: u32,
    /// Stencil bits.
    ///
    /// ANGLE's own context configuration asks for eight of these alongside
    /// depth twenty-four and refuses to initialise when nothing answers, so a
    /// row without them is invisible to it however else it matches.
    pub stencil: u32,
}

impl XGlxFbConfig {
    /// The X depth a drawable of this configuration reports.
    ///
    /// A pure conversion of the row rather than a second table, so a drawable's
    /// depth and the depth advertised for its configuration cannot disagree.
    pub const fn depth(self) -> u8 {
        24 + self.alpha as u8
    }

    /// `GLX_BIND_TO_TEXTURE_RGB_EXT` as advertised for this configuration.
    ///
    /// Advertisement and constructor validation read this one answer, so a
    /// format the catalog promises cannot be refused by the constructor, and
    /// one it withholds cannot be accepted. Binding capability is what the
    /// extension tests, not the alpha depth.
    pub const fn bind_to_texture_rgb(self) -> bool {
        true
    }

    /// `GLX_BIND_TO_TEXTURE_RGBA_EXT` as advertised for this configuration.
    pub const fn bind_to_texture_rgba(self) -> bool {
        true
    }

    /// `GLX_BIND_TO_MIPMAP_TEXTURE_EXT` as advertised for this configuration.
    ///
    /// Advertised explicitly as false rather than omitted: a driver comparing
    /// attributes for equality distinguishes an absent one from a false one.
    pub const fn bind_to_mipmap_texture(self) -> bool {
        false
    }

    /// Whether this configuration can present a pixmap in a texture format.
    ///
    /// `NONE` binds no texture, which any configuration can do. The others are
    /// admitted exactly where the corresponding capability is advertised.
    pub const fn admits_texture_format(self, format: u32) -> bool {
        match format {
            crate::X_GLX_TEXTURE_FORMAT_NONE_VALUE => true,
            crate::X_GLX_TEXTURE_FORMAT_RGB_VALUE => self.bind_to_texture_rgb(),
            crate::X_GLX_TEXTURE_FORMAT_RGBA_VALUE => self.bind_to_texture_rgba(),
            _ => false,
        }
    }

    /// The format Sophia gives a pixmap whose client named none.
    ///
    /// A choice rather than a requirement: the extension's Offscreen Rendering
    /// paragraph does not state a default, so this prefers the richer binding
    /// the configuration advertises and falls back to what it will take.
    pub const fn default_texture_format(self) -> u32 {
        if self.bind_to_texture_rgba() {
            crate::X_GLX_TEXTURE_FORMAT_RGBA_VALUE
        } else if self.bind_to_texture_rgb() {
            crate::X_GLX_TEXTURE_FORMAT_RGB_VALUE
        } else {
            crate::X_GLX_TEXTURE_FORMAT_NONE_VALUE
        }
    }
}

/// The configurations Sophia offers, in reply order.
///
/// The first three are the original rows and keep their identifiers and their
/// answers exactly. The last three mirror them with a stencil buffer, and are
/// offered only where pixmap textures are supported, so a server without them
/// answers byte for byte what it always did.
pub const X_GLX_FB_CONFIGS: [XGlxFbConfig; 6] = [
    XGlxFbConfig {
        id: 1,
        visual: crate::X_SETUP_DEFAULT_VISUAL,
        alpha: 0,
        srgb: 0,
        stencil: 0,
    },
    XGlxFbConfig {
        id: 2,
        visual: crate::X_SETUP_ARGB_VISUAL,
        alpha: 8,
        srgb: 0,
        stencil: 0,
    },
    XGlxFbConfig {
        id: 3,
        visual: crate::X_SETUP_ARGB_VISUAL,
        alpha: 8,
        srgb: 1,
        stencil: 0,
    },
    XGlxFbConfig {
        id: 4,
        visual: crate::X_SETUP_DEFAULT_VISUAL,
        alpha: 0,
        srgb: 0,
        stencil: 8,
    },
    XGlxFbConfig {
        id: 5,
        visual: crate::X_SETUP_ARGB_VISUAL,
        alpha: 8,
        srgb: 0,
        stencil: 8,
    },
    XGlxFbConfig {
        id: 6,
        visual: crate::X_SETUP_ARGB_VISUAL,
        alpha: 8,
        srgb: 1,
        stencil: 8,
    },
];

/// How many configurations a server without pixmap textures offers.
pub const X_GLX_BASE_FB_CONFIG_COUNT: usize = 3;

/// The configurations a client may see.
pub fn x_glx_fb_configs(pixmap_textures: bool) -> &'static [XGlxFbConfig] {
    if pixmap_textures {
        &X_GLX_FB_CONFIGS
    } else {
        &X_GLX_FB_CONFIGS[..X_GLX_BASE_FB_CONFIG_COUNT]
    }
}

/// The largest offscreen surface Sophia will record.
///
/// Sophia allocates nothing for a pbuffer, so this is a refusal threshold rather
/// than a capability claim. It is published as `GLX_MAX_PBUFFER_*` from the same
/// constants that enforce it, so the advertisement cannot drift from the answer.
pub const X_GLX_MAX_PBUFFER_WIDTH: u32 = 4096;
pub const X_GLX_MAX_PBUFFER_HEIGHT: u32 = 4096;
pub const X_GLX_MAX_PBUFFER_PIXELS: u32 = X_GLX_MAX_PBUFFER_WIDTH * X_GLX_MAX_PBUFFER_HEIGHT;

/// `GLX_DRAWABLE_TYPE`: the drawable kinds these configurations support.
///
/// Window and pbuffer, and exactly the kinds Sophia implements on its own.
pub const X_GLX_DRAWABLE_TYPE_MASK: u32 = 0x5;

/// The same, plus `GLX_PIXMAP_BIT`, where a provider backs pixmap textures.
///
/// Pixmap and pbuffer have to appear on ONE configuration rather than on two:
/// a client that derives EGL configurations from these rows emits its
/// bind-to-texture variant only when it finds both together.
pub const X_GLX_DRAWABLE_TYPE_MASK_WITH_PIXMAPS: u32 = 0x7;

/// `GLX_DRAWABLE_TYPE` for the advertised capability.
pub const fn x_glx_drawable_type_mask(pixmap_textures: bool) -> u32 {
    if pixmap_textures {
        X_GLX_DRAWABLE_TYPE_MASK_WITH_PIXMAPS
    } else {
        X_GLX_DRAWABLE_TYPE_MASK
    }
}

/// The configuration a client named, if Sophia offers it to that client.
pub fn x_glx_fb_config(id: u32, pixmap_textures: bool) -> Option<XGlxFbConfig> {
    x_glx_fb_configs(pixmap_textures)
        .iter()
        .copied()
        .find(|config| config.id == id)
}

/// How a GLX pixmap presents itself as a texture.
///
/// Settled once at creation and answered back verbatim, because the extension
/// defines these as queryable attributes of the drawable rather than as hints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XGlxPixmapTexture {
    /// One of the target bits, not the target name the client sent.
    pub target: u32,
    pub format: u32,
    pub mipmap: bool,
}
