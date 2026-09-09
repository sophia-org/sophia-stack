impl XAuthorityRuntime {
    pub fn create_glx_context(
        &mut self,
        namespace: NamespaceId,
        context: crate::XResourceId,
        fbconfig: u32,
        direct: bool,
    ) -> Result<(), XAuthorityRuntimeError> {
        if self.resources.get(context).is_some()
            || self.glx_contexts.contains_key(&context)
            || self.glx_drawables.contains_key(&context)
        {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        self.glx_contexts
            .insert(context, (namespace, fbconfig, direct));
        Ok(())
    }

    pub fn glx_context(
        &self,
        namespace: NamespaceId,
        context: crate::XResourceId,
    ) -> Result<(u32, bool), XAuthorityRuntimeError> {
        self.glx_contexts
            .get(&context)
            .filter(|(owner, _, _)| *owner == namespace)
            .map(|(_, config, direct)| (*config, *direct))
            .ok_or(XAuthorityRuntimeError::UnknownResource)
    }

    pub fn destroy_glx_context(
        &mut self,
        namespace: NamespaceId,
        context: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.glx_context(namespace, context)?;
        self.glx_contexts.remove(&context);
        Ok(())
    }

    pub fn create_glx_window(
        &mut self,
        namespace: NamespaceId,
        glx_window: crate::XResourceId,
        window: crate::XResourceId,
        fbconfig: u32,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.validate_window_access(namespace, window)?;
        if self.resources.get(glx_window).is_some()
            || self.glx_contexts.contains_key(&glx_window)
            || self.glx_drawables.contains_key(&glx_window)
        {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        self.glx_drawables.insert(
            glx_window,
            XGlxDrawableRecord {
                owner: namespace,
                fbconfig,
                backing: XGlxDrawableBacking::Window(window),
            },
        );
        Ok(())
    }

    pub fn glx_drawable(
        &self,
        namespace: NamespaceId,
        drawable: crate::XResourceId,
    ) -> Result<(crate::XResourceId, u32), XAuthorityRuntimeError> {
        if let Some(record) = self.glx_drawables.get(&drawable) {
            if record.owner != namespace {
                return Err(XAuthorityRuntimeError::UnknownResource);
            }
            return match record.backing {
                XGlxDrawableBacking::Window(window) => Ok((window, record.fbconfig)),
                // Offscreen drawables have no underlying X window.
                XGlxDrawableBacking::Pbuffer(_) | XGlxDrawableBacking::Pixmap { .. } => {
                    Err(XAuthorityRuntimeError::WrongResourceKind)
                }
            };
        }
        self.validate_window_access(namespace, drawable)?;
        // Plain X windows select a configuration through their visual.
        let visual = self.window_visual(drawable).1;
        // Stencil variants share visuals; implicit window configs use the base rows.
        let config = crate::X_GLX_FB_CONFIGS[..crate::X_GLX_BASE_FB_CONFIG_COUNT]
            .iter()
            .rev()
            .find(|config| config.visual == visual)
            .map_or(1, |config| config.id);
        Ok((drawable, config))
    }

    /// Records a pbuffer extent; direct clients own its rendering storage.
    pub fn create_glx_pbuffer(
        &mut self,
        namespace: NamespaceId,
        pbuffer: crate::XResourceId,
        fbconfig: u32,
        size: Size,
    ) -> Result<(), XAuthorityRuntimeError> {
        if self.resources.get(pbuffer).is_some()
            || self.glx_contexts.contains_key(&pbuffer)
            || self.glx_drawables.contains_key(&pbuffer)
        {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        self.glx_drawables.insert(
            pbuffer,
            XGlxDrawableRecord {
                owner: namespace,
                fbconfig,
                backing: XGlxDrawableBacking::Pbuffer(size),
            },
        );
        Ok(())
    }

    /// The extent and configuration of an offscreen drawable this client owns.
    pub fn glx_pbuffer(
        &self,
        namespace: NamespaceId,
        pbuffer: crate::XResourceId,
    ) -> Result<(Size, u32), XAuthorityRuntimeError> {
        let record = self
            .glx_drawables
            .get(&pbuffer)
            .filter(|record| record.owner == namespace)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        match record.backing {
            XGlxDrawableBacking::Pbuffer(size) => Ok((size, record.fbconfig)),
            XGlxDrawableBacking::Window(_) | XGlxDrawableBacking::Pixmap { .. } => {
                Err(XAuthorityRuntimeError::WrongResourceKind)
            }
        }
    }

    /// Wraps RGB visual storage or a full RGBA color buffer of the configuration.
    pub fn create_glx_pixmap(
        &mut self,
        namespace: NamespaceId,
        glx_pixmap: crate::XResourceId,
        config: crate::XGlxFbConfig,
        pixmap: crate::XResourceId,
        requested_target: Option<u32>,
        requested_format: Option<u32>,
        requested_mipmap: Option<bool>,
    ) -> Result<(), XAuthorityRuntimeError> {
        // Graphics contexts also reserve IDs outside the main resource table.
        if self.resources.get(glx_pixmap).is_some()
            || self.glx_contexts.contains_key(&glx_pixmap)
            || self.glx_drawables.contains_key(&glx_pixmap)
            || self.graphics_context_values(namespace, glx_pixmap).is_ok()
        {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        self.validate_pixmap_access(namespace, pixmap)?;
        let record = self
            .pixmaps
            .get(&pixmap)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        if record.depth != config.visual_depth && record.depth != config.color_bits() {
            return Err(XAuthorityRuntimeError::WrongResourceKind);
        }
        // Texture attributes are fixed against the backing extent at creation.
        let (width, height) = (record.size.width, record.size.height);
        let target = match requested_target {
            Some(named) => crate::x_glx_texture_target_bit(named)
                .filter(|bit| crate::X_GLX_TEXTURE_TARGETS_ALL & bit != 0)
                .filter(|bit| crate::x_glx_texture_target_admits(*bit, width, height))
                .ok_or(XAuthorityRuntimeError::InvalidResource)?,
            None => {
                crate::x_glx_default_texture_target(crate::X_GLX_TEXTURE_TARGETS_ALL, width, height)
                    .ok_or(XAuthorityRuntimeError::InvalidResource)?
            }
        };
        // Validation uses the capabilities advertised by this configuration.
        let format = requested_format.unwrap_or_else(|| config.default_texture_format());
        if !config.admits_texture_format(format) {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        // Mipmap requests require an advertised mipmap binding capability.
        let mipmap = requested_mipmap.unwrap_or(false);
        if mipmap && !config.bind_to_mipmap_texture() {
            return Err(XAuthorityRuntimeError::InvalidResource);
        }
        self.glx_drawables.insert(
            glx_pixmap,
            XGlxDrawableRecord {
                owner: namespace,
                fbconfig: config.id,
                backing: XGlxDrawableBacking::Pixmap {
                    pixmap,
                    texture: crate::XGlxPixmapTexture {
                        target,
                        format,
                        mipmap,
                    },
                },
            },
        );
        Ok(())
    }

    /// The pixmap and configuration of a GLX pixmap this client owns.
    pub fn glx_pixmap(
        &self,
        namespace: NamespaceId,
        glx_pixmap: crate::XResourceId,
    ) -> Result<(crate::XResourceId, u32), XAuthorityRuntimeError> {
        let record = self
            .glx_drawables
            .get(&glx_pixmap)
            .filter(|record| record.owner == namespace)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        match record.backing {
            XGlxDrawableBacking::Pixmap { pixmap, .. } => Ok((pixmap, record.fbconfig)),
            XGlxDrawableBacking::Window(_) | XGlxDrawableBacking::Pbuffer(_) => {
                Err(XAuthorityRuntimeError::WrongResourceKind)
            }
        }
    }

    /// Resolves window, pbuffer and pixmap configurations for context binding.
    pub fn glx_drawable_config(
        &self,
        namespace: NamespaceId,
        drawable: crate::XResourceId,
    ) -> Result<u32, XAuthorityRuntimeError> {
        if let Some(record) = self
            .glx_drawables
            .get(&drawable)
            .filter(|record| record.owner == namespace)
        {
            return Ok(record.fbconfig);
        }
        // A plain X window renders through the configuration its visual names.
        self.glx_drawable(namespace, drawable)
            .map(|(_, config)| config)
    }

    /// Queries the live or retained backing after the original pixmap XID is freed.
    pub(crate) fn glx_pixmap_geometry(
        &self,
        namespace: NamespaceId,
        glx_pixmap: crate::XResourceId,
    ) -> Result<(Size, u8), XAuthorityRuntimeError> {
        let (backing, _) = self.glx_pixmap(namespace, glx_pixmap)?;
        let pixmap = self
            .pixmaps
            .get(&backing)
            .or_else(|| {
                self.retained_pixmap_backings
                    .get(&backing)
                    .map(|held| &held.pixmap)
            })
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        Ok((pixmap.size, pixmap.depth))
    }

    /// Texture attributes stay attached to the retained pixmap identity.
    pub fn glx_pixmap_attributes(
        &self,
        namespace: NamespaceId,
        glx_pixmap: crate::XResourceId,
    ) -> Result<(Size, u32, crate::XGlxPixmapTexture), XAuthorityRuntimeError> {
        let record = self
            .glx_drawables
            .get(&glx_pixmap)
            .filter(|record| record.owner == namespace)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        let XGlxDrawableBacking::Pixmap { pixmap, texture } = record.backing else {
            return Err(XAuthorityRuntimeError::WrongResourceKind);
        };
        let size = self
            .pixmaps
            .get(&pixmap)
            .map(|record| record.size)
            .or_else(|| {
                self.retained_pixmap_backings
                    .get(&pixmap)
                    .map(|retained| retained.pixmap.size)
            })
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        Ok((size, record.fbconfig, texture))
    }

    pub fn destroy_glx_pixmap(
        &mut self,
        namespace: NamespaceId,
        glx_pixmap: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        let (backing, _) = self.glx_pixmap(namespace, glx_pixmap)?;
        self.glx_drawables.remove(&glx_pixmap);
        // A freed pixmap survives until its last retained referent is released.
        self.release_retained_referent(backing, false);
        Ok(())
    }

    pub fn destroy_glx_pbuffer(
        &mut self,
        namespace: NamespaceId,
        pbuffer: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        self.glx_pbuffer(namespace, pbuffer)?;
        self.retire_pixmap_export_drawable(pbuffer);
        self.glx_drawables.remove(&pbuffer);
        Ok(())
    }

    pub fn destroy_glx_window(
        &mut self,
        namespace: NamespaceId,
        glx_window: crate::XResourceId,
    ) -> Result<(), XAuthorityRuntimeError> {
        // Destruction requires a GLX alias; plain X windows are not aliases.
        let record = self
            .glx_drawables
            .get(&glx_window)
            .filter(|record| record.owner == namespace)
            .ok_or(XAuthorityRuntimeError::UnknownResource)?;
        if !matches!(record.backing, XGlxDrawableBacking::Window(_)) {
            return Err(XAuthorityRuntimeError::WrongResourceKind);
        }
        self.glx_drawables.remove(&glx_window);
        Ok(())
    }
}
