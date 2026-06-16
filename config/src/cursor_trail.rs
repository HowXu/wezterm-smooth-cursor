use wezterm_dynamic::{FromDynamic, ToDynamic};

#[derive(Debug, Clone, FromDynamic, ToDynamic)]
pub struct CursorTrailConfig {
    /// Enable the cursor trail effect.
    #[dynamic(default)]
    pub enabled: bool,

    /// Cursor trail dwell time in milliseconds.
    ///
    /// The default of 0 updates the trail target immediately when the cursor
    /// moves. Larger values delay the target update until the cursor has been
    /// stationary for this long.
    #[dynamic(default)]
    pub dwell_threshold: u64,

    /// Animation duration in milliseconds for the leading edge to reach the
    /// cursor.
    #[dynamic(default = "default_duration")]
    pub duration: u64,

    /// Duration multiplier for the trailing edge. Higher values produce a
    /// longer smear.
    #[dynamic(default = "default_spread")]
    pub spread: f32,

    /// Minimum cursor movement, in cells, needed to draw the trail.
    #[dynamic(default = "default_distance_threshold")]
    pub distance_threshold: usize,

    /// Opacity for the cursor trail.
    #[dynamic(default = "default_opacity")]
    pub opacity: f32,
}

impl CursorTrailConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.duration == 0 {
            return Err("cursor_trail.duration must be greater than 0".to_string());
        }

        if self.spread < 1.0 {
            return Err(format!(
                "cursor_trail.spread must be at least 1.0 (got {})",
                self.spread
            ));
        }

        if !(0.0..=1.0).contains(&self.opacity) {
            return Err(format!(
                "cursor_trail.opacity must be between 0.0 and 1.0 (got {})",
                self.opacity
            ));
        }

        Ok(())
    }
}

impl Default for CursorTrailConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dwell_threshold: 0,
            duration: default_duration(),
            spread: default_spread(),
            distance_threshold: default_distance_threshold(),
            opacity: default_opacity(),
        }
    }
}

fn default_duration() -> u64 {
    80
}

fn default_spread() -> f32 {
    2.6
}

fn default_distance_threshold() -> usize {
    1
}

fn default_opacity() -> f32 {
    0.55
}
