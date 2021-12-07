const LAB_E: f64 =   216.0 / 24389.0;
const LAB_K: f64 = 24389.0 / 27.0;
const R_X: f64 = 0.9505;
const R_Y: f64 = 1.0000;
const R_Z: f64 = 1.0891;

const SRGBI_MATRIX: [f64;9] = [
     3.2404542, -1.5371385, -0.4985314,
    -0.9692660,  1.8760108,  0.0415560,
     0.0556434, -0.2040259,  1.0572252
];

/// Multiples an array of three `f64`'s by a 3x3 array of `f64`
fn matrix_multiply3_3x3(thr: &[f64;3], mat: &[f64;9]) -> [f64;3] {
    [
        thr[0] * mat[0] + thr[1] * mat[1] + thr[2] * mat[2],
        thr[0] * mat[3] + thr[1] * mat[4] + thr[2] * mat[5],
        thr[0] * mat[6] + thr[1] * mat[7] + thr[2] * mat[8],
    ]
}

/// A representation of color via the LCH color spectrum, which is the LAB color space in a polar coordinate system.                      
pub struct LCH {
    /// Preceptual Lightness
    pub l: f64,
    /// Chroma, or how strong the 'pigmentation' is within a color.
    pub c: f64,
    /// Hue, or what tone the color is.
    pub h: f64,
}

/// A representation of color via preceptual lightness, and two opponent color axis.
pub struct LAB {
    ///Preceptual Lightness
    pub l: f64,
    ///green-red axis, where negative values are more green, positive values are more red.
    pub a: f64,
    ///blue-yellow axis, where negative values are more blue, and positive values are more yellow.
    pub b: f64,
}

/// A representation of color via tristimulus values, which approximate human vision.
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

///A representation of color via the three channels (or axis) red, green, and blue.
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}


impl RGB {
    /// Creates a new instance of an RGB struct.
    pub fn new(r: f64, g: f64, b: f64) -> RGB {
        RGB {
            r: r,
            g: g,
            b: b,
        }
    }
    /// Converts from standard 24-bit (`[u8;3]`) representation, to an RGB struct in f64.
    /// ```
    /// use color_picky::RGB;
    /// 
    /// let black = RGB::from_bytes(0,0,0);
    /// let white = RGB::from_bytes(255, 255, 255);
    /// ```
    pub fn from_bytes(r: u8, g: u8, b: u8) -> RGB {
        RGB::new(
            (r as f64) / 255.0,
            (g as f64) / 255.0,
            (b as f64) / 255.0,
        )
    }
    /// Checks whether all the components of an RGB value are within the `0.0..1.0` range for valid representation.
    /// ```
    /// use color_picky::RGB;
    /// 
    /// let valid = RGB::new(0.0, 0.1, 1.0);
    /// let too_low = RGB::new(-0.1, 0.0, 0.0);
    /// let too_high = RGB::new(1.1, 0.0, 0.0);
    /// 
    /// assert!(!RGB::within_gamut(&too_low));
    /// assert!(!RGB::within_gamut(&too_high));
    /// assert!(RGB::within_gamut(&valid));
    /// ```
    pub fn within_gamut(&self) -> bool {
        if self.r > 1.0 || self.r < 0.0 ||
            self.g > 1.0 || self.g < 0.0 ||
            self.b > 1.0 || self.b < 0.0 {
            return false
        } else {
            return true
        }
    }
    /// Converts a floating point RGB color to a traditional 24-bit color value.
    /// 
    /// ```
    /// use color_picky::RGB;
    /// let white = RGB::new(0.9999, 0.9999, 0.9999);
    /// let white2 = RGB::new(1.0, 1.0, 1.0);
    /// 
    /// assert_eq!(white.to_bytes(), [255, 255, 255]);
    /// assert_eq!(white2.to_bytes(), [255, 255, 255]);
    /// ```
    /// 
    pub fn to_bytes(&self) -> [u8;3] {
        [((self.r * 255.0).round() as u8), 
        ((self.g * 255.0).round() as u8),
        ((self.b * 255.0).round() as u8),
        ]
    }

}

impl XYZ {
    ///Creates a new XYZ instance.
    pub fn new(x: f64, y:f64 , z:f64) -> XYZ {
        XYZ{
            x:x,
            y:y,
            z:z,
        }
    }

    pub fn to_rgb(&self) -> RGB {
        let l_rgb = matrix_multiply3_3x3(&[self.x, self.y, self.z], &SRGBI_MATRIX);
        
        let l_r = l_rgb[0] * LAB_K/100.0;
        let l_g = l_rgb[1] * LAB_K/100.0;
        let l_b = l_rgb[2] * LAB_K/100.0;

        let h_r = (l_r.cbrt() * 1.16) - 0.16;
        let h_g = (l_g.cbrt() * 1.16) - 0.16;
        let h_b = (l_b.cbrt() * 1.16) - 0.16;

        let r_r = if l_r <= LAB_E {l_r} else {h_r};
        let r_g = if l_g <= LAB_E {l_g} else {h_g};
        let r_b = if l_b <= LAB_E {l_b} else {h_b};
        
        RGB::new(r_r, r_g, r_b)

    }

}

impl LAB {
    pub fn new(l: f64, a: f64, b: f64) -> LAB {
        return LAB {
            l: l,
            a: a,
            b: b,
        }
    }
    pub fn to_xyz(&self) -> XYZ {

        let f_y = (self.l + 16.0) / 116.0;
        let f_x = (self.a / 500.0) + f_y;
        let f_z = (self.b / 200.0) + f_y;

        let f_x = f_x * f_x * f_x;
        let f_y = f_y * f_y * f_y;
        let f_z = f_z * f_z * f_z;

        let l_x = (116.0 * f_x - 16.0) / LAB_K;
        let l_y = (self.l / LAB_K)     / LAB_K;
        let l_z = (116.0 * f_z - 16.0) / LAB_K;

        let d_x  = if     f_x <= LAB_E {l_x} else {f_x};
        let d_y  = if self.l <= LAB_E {l_y} else {f_y};
        let d_z  = if     f_z <= LAB_E {l_z} else {f_z};

        XYZ::new(R_X * d_x, R_Y * d_y, R_Z * d_z)
    }
}

impl LCH {
    pub fn new(l: f64, c: f64, h: f64) -> LCH {
        LCH {
            l: l,
            c: c,
            h: h,
        }
    } 

    pub fn to_lab(&self) -> LAB {
        let rads = std::f64::consts::PI / 180.0 * self.h; //Simplified from hue/360.0 = rads/2*Pi
        LAB::new(self.l, rads.cos() * self.c ,rads.sin() * self.c)
    }
}