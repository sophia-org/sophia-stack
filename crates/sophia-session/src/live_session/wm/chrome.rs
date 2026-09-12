impl LiveWmSession {
    fn surface_chrome_style(&self) -> Option<sophia_engine::SurfaceChromeStyle> {
        Some(self.visual_chrome)
    }

    fn indicator_publication(&self) -> Option<sophia_engine::PolicyIndicatorPublication> {
        self.public
            .as_ref()
            .map(|public| public.reducer.indicator_publication())
    }

    /// The output the user is on, as committed public policy holds it.
    ///
    /// Not the startup activation plan's focused output, which records where
    /// the topology began rather than where focus is now, and not an inference
    /// from a focused application: an active output holding no window has no
    /// seat focus to infer from, which is the case a bar most needs to show.
    fn active_output(&self) -> Option<sophia_protocol::OutputId> {
        self.public.as_ref().map(|public| public.active_output)
    }

    fn candidate_chrome_style(&self) -> sophia_engine::SurfaceChromeStyle {
        if self.wm_chrome_supported {
            PersistentXtermSessionConfig::wm_surface_chrome_style(self.chrome)
        } else {
            self.fallback_chrome
        }
    }

    fn stage_visual_chrome(&mut self, candidate: sophia_engine::SurfaceChromeStyle) {
        if self.committed == 0 || candidate.clearance() == self.visual_chrome.clearance() {
            self.visual_chrome = candidate;
            self.pending_visual_chrome = None;
        } else {
            self.pending_visual_chrome = Some(candidate);
            self.work_area_relayout_required = true;
        }
    }

    fn set_fallback_chrome(&mut self, style: sophia_engine::SurfaceChromeStyle) {
        self.fallback_chrome = style;
        if !self.wm_chrome_supported {
            self.stage_visual_chrome(style);
        }
    }
}
