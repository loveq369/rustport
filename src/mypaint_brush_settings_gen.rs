#[repr(C)]
pub enum MyPaintBrushInput {
    MYPAINT_BRUSH_INPUT_PRESSURE = 0,
    MYPAINT_BRUSH_INPUT_SPEED1,
    MYPAINT_BRUSH_INPUT_SPEED2,
    MYPAINT_BRUSH_INPUT_RANDOM,
    MYPAINT_BRUSH_INPUT_STROKE,
    MYPAINT_BRUSH_INPUT_DIRECTION,
    MYPAINT_BRUSH_INPUT_TILT_DECLINATION,
    MYPAINT_BRUSH_INPUT_TILT_ASCENSION,
    MYPAINT_BRUSH_INPUT_CUSTOM,
    // note: this is a trick used by the c code which doesn't work in rust due
    // to the fact that enum varians aren't constant expressions
    // MYPAINT_BRUSH_INPUTS_COUNT
}

#[repr(C)]
pub enum MyPaintBrushSetting {
    MYPAINT_BRUSH_SETTING_OPAQUE = 0,
    MYPAINT_BRUSH_SETTING_OPAQUE_MULTIPLY,
    MYPAINT_BRUSH_SETTING_OPAQUE_LINEARIZE,
    MYPAINT_BRUSH_SETTING_RADIUS_LOGARITHMIC,
    MYPAINT_BRUSH_SETTING_HARDNESS,
    MYPAINT_BRUSH_SETTING_ANTI_ALIASING,
    MYPAINT_BRUSH_SETTING_DABS_PER_BASIC_RADIUS,
    MYPAINT_BRUSH_SETTING_DABS_PER_ACTUAL_RADIUS,
    MYPAINT_BRUSH_SETTING_DABS_PER_SECOND,
    MYPAINT_BRUSH_SETTING_RADIUS_BY_RANDOM,
    MYPAINT_BRUSH_SETTING_SPEED1_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SPEED2_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SPEED1_GAMMA,
    MYPAINT_BRUSH_SETTING_SPEED2_GAMMA,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_RANDOM,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED,
    MYPAINT_BRUSH_SETTING_OFFSET_BY_SPEED_SLOWNESS,
    MYPAINT_BRUSH_SETTING_SLOW_TRACKING,
    MYPAINT_BRUSH_SETTING_SLOW_TRACKING_PER_DAB,
    MYPAINT_BRUSH_SETTING_TRACKING_NOISE,
    MYPAINT_BRUSH_SETTING_COLOR_H,
    MYPAINT_BRUSH_SETTING_COLOR_S,
    MYPAINT_BRUSH_SETTING_COLOR_V,
    MYPAINT_BRUSH_SETTING_RESTORE_COLOR,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_H,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_L,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSL_S,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_V,
    MYPAINT_BRUSH_SETTING_CHANGE_COLOR_HSV_S,
    MYPAINT_BRUSH_SETTING_SMUDGE,
    MYPAINT_BRUSH_SETTING_SMUDGE_LENGTH,
    MYPAINT_BRUSH_SETTING_SMUDGE_RADIUS_LOG,
    MYPAINT_BRUSH_SETTING_ERASER,
    MYPAINT_BRUSH_SETTING_STROKE_THRESHOLD,
    MYPAINT_BRUSH_SETTING_STROKE_DURATION_LOGARITHMIC,
    MYPAINT_BRUSH_SETTING_STROKE_HOLDTIME,
    MYPAINT_BRUSH_SETTING_CUSTOM_INPUT,
    MYPAINT_BRUSH_SETTING_CUSTOM_INPUT_SLOWNESS,
    MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_RATIO,
    MYPAINT_BRUSH_SETTING_ELLIPTICAL_DAB_ANGLE,
    MYPAINT_BRUSH_SETTING_DIRECTION_FILTER,
    MYPAINT_BRUSH_SETTING_LOCK_ALPHA,
    MYPAINT_BRUSH_SETTING_COLORIZE,
    MYPAINT_BRUSH_SETTING_SNAP_TO_PIXEL,
    MYPAINT_BRUSH_SETTING_PRESSURE_GAIN_LOG,
    // MYPAINT_BRUSH_SETTINGS_COUNT
}

/// direct structification of the previous enum of the same name
#[derive(Copy, Clone, Default)]
pub struct MyPaintBrushState {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub partial_dabs: f32,
    pub actual_radius: f32,
    pub smudge_ra: f32,
    pub smudge_ga: f32,
    pub smudge_ba: f32,
    pub smudge_a: f32,
    pub last_getcolor_r: f32,
    pub last_getcolor_g: f32,
    pub last_getcolor_b: f32,
    pub last_getcolor_a: f32,
    pub last_getcolor_recentness: f32,
    pub actual_x: f32,
    pub actual_y: f32,
    pub norm_dx_slow: f32,
    pub norm_dy_slow: f32,
    pub norm_speed1_slow: f32,
    pub norm_speed2_slow: f32,
    pub stroke: f32,
    pub stroke_started: f32,
    pub custom_input: f32,
    pub rng_seed: f32,
    pub actual_elliptical_dab_ratio: f32,
    pub actual_elliptical_dab_angle: f32,
    pub direction_dx: f32,
    pub direction_dy: f32,
    pub declination: f32,
    pub ascension: f32
}

impl MyPaintBrushState {
    pub fn int_to_state(&mut self, int: usize) -> &mut f32 {
        match int {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.pressure,
            3 => &mut self.partial_dabs,
            4 => &mut self.actual_radius,
            5 => &mut self.smudge_ra,
            6 => &mut self.smudge_ga,
            7 => &mut self.smudge_ba,
            8 => &mut self.smudge_a,
            9 => &mut self.last_getcolor_r,
            10 => &mut self.last_getcolor_g,
            11 => &mut self.last_getcolor_b,
            12 => &mut self.last_getcolor_a,
            13 => &mut self.last_getcolor_recentness,
            14 => &mut self.actual_x,
            15 => &mut self.actual_y,
            16 => &mut self.norm_dx_slow,
            17 => &mut self.norm_dy_slow,
            18 => &mut self.norm_speed1_slow,
            19 => &mut self.norm_speed2_slow,
            20 => &mut self.stroke,
            21 => &mut self.stroke_started,
            22 => &mut self.custom_input,
            23 => &mut self.rng_seed,
            24 => &mut self.actual_elliptical_dab_ratio,
            25 => &mut self.actual_elliptical_dab_angle,
            26 => &mut self.direction_dx,
            27 => &mut self.direction_dy,
            28 => &mut self.declination,
            29 => &mut self.ascension,
            _ => panic!(),
        }
    }
}
