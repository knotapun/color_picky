// const float labE = 216.0/24389.0;
// const float labK = 24389.0/27.0;
// const float pi = 3.14159;
// const vec3 rXYZ = vec3 (0.9505, 1.0000, 1.0891);
// const mat3 sRGBiMatrix = mat3 (
//    3.2404542, -1.5371385, -0.4985314,
//   -0.9692660,  1.8760108,  0.0415560,
//    0.0556434, -0.2040259,  1.0572252
// );

// #define NEWTON_ITER 2
// #define HALLEY_ITER 2

// //Stolen from Scholarius: https://www.shadertoy.com/view/wts3RX 
// //Turns out, cube roots are kind of annoying to calculate.
// float cbrt( float x )
// {
// 	float y = sign(x) * uintBitsToFloat( floatBitsToUint( abs(x) ) / 3u + 0x2a514067u );

// 	for( int i = 0; i < NEWTON_ITER; ++i )
//     	y = ( 2. * y + x / ( y * y ) ) * .333333333;

//     for( int i = 0; i < HALLEY_ITER; ++i )
//     {
//     	float y3 = y * y * y;
//         y *= ( y3 + 2. * x ) / ( 2. * y3 + x );
//     }
    
//     return y;
// }

// vec3 LCHtoLAB(vec3 LCH) {
//     float rads = radians(LCH.z);
//     vec3 LAB = vec3(LCH[0], cos(rads) * LCH[1],sin(rads) * LCH[1]);
//     return LAB;
// }

// vec3 LABtoXYZ(vec3 LAB) {

//     float fY = (LAB[0] + 16.0) / 116.0;
    
//     vec3 fXYZ = vec3( 
//          LAB[1]/500.0 + fY,
//          fY,
//         -LAB[2]/200.0 + fY
//     );
    
//     fXYZ = fXYZ * fXYZ * fXYZ;
    
    
//     vec3 lowXYZ = vec3(
//         116.0 * fXYZ[0] - 16.0,
//         LAB[0]/labK,
//         116.0 * fXYZ[2] - 16.0
//     ) / vec3(labK); 
    
//     vec3 dXYZ = vec3(
//     ((fXYZ[0] <= labE)          ? lowXYZ[0] : fXYZ[0]),
//     (( LAB[0] <= (labK * labE)) ? lowXYZ[1] : fXYZ[1]),
//     ((fXYZ[2] <= labE)          ? lowXYZ[2] : fXYZ[2])
//     );
//     return rXYZ * dXYZ;
// }

// vec3 XYZtoRGB(vec3 XYZ) {
//     vec3 lRGB = XYZ * sRGBiMatrix;
//     vec3 lowRGB = lRGB * labK/100.0;
//     vec3 highRGB = (vec3(
//         cbrt(lRGB[0]),
//         cbrt(lRGB[1]),
//         cbrt(lRGB[2])
//     ) * 1.16) - vec3(0.16);
//     vec3 RGB = vec3(
//     (lRGB[0] <= labE) ? lowRGB[0] : highRGB[0],
//     (lRGB[1] <= labE) ? lowRGB[1] : highRGB[1], 
//     (lRGB[2] <= labE) ? lowRGB[2] : highRGB[2]
//   );
//   return RGB;
// }

// vec3 coordsToLCH(vec2 fragCoord, vec4 iMouse, vec3 iResolution) {
//   vec3 coords = vec3(
//        iMouse.x / iResolution.x,
//     fragCoord.y / iResolution.y,
//     fragCoord.x / iResolution.x
//   );
//   vec3 scaling = vec3(100.0, 128.0, 360.0);
//   return coords * scaling;
// }

// vec3 trimByValue(vec3 RGB, vec2 fragCoord) {
//   if ( RGB.r < 0.0 || RGB.r > 1.0 || RGB.g < 0.0 || RGB.g > 1.0 || RGB.b < 0.0 || RGB.b > 1.0) {
//     bool horizontalMod = mod(fragCoord.x, 20.0) > 10.0;
//     bool verticalMod = mod(fragCoord.y, 20.0) > 10.0;
  
//       if((horizontalMod && !verticalMod) || (!horizontalMod && verticalMod)) {
//         RGB = vec3(0.5);
//       } else {
//         RGB = vec3(0.7);
//     }
//   }
//   return RGB;
// }

// /*  WIP 
//     attempts to draw circles of the color under the mouse via shader.
//     Will need to be called early on, and will work by manipulating what
//     calls further down the line think the actual coordinates of the pixels within the
//     cicle are the coordinates of the mouse itself, meaning their color will just be 
//     calculated to be the same as that pixel.
// */
// vec3 addMouseHighlight(vec3 RGB, vec2 fragCoord, vec4 iMouse) {
//     vec2 mouseLocation =    iMouse.xy / iResolution.xy;
//     vec2 pixelLocation = fragCoord.xy / iResolution.xy;
//     vec2 dist = mouseLocation - pixelLocation;
//     float dis = sqrt(dist.x * dist.x + dist.y * dist.y);
//     if(dis > 10.0){
//         RGB = vec3(0.5);
//     }
//     return RGB;
// }

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
  
//   vec3 LCH = coordsToLCH(fragCoord, iMouse, iResolution);
    
//   vec3 LAB = LCHtoLAB(LCH);
   
//   vec3 XYZ = LABtoXYZ(LAB);
  
//   vec3 RGB = XYZtoRGB(XYZ);
  
//   vec3 TRIMMED = trimByValue(RGB, fragCoord);
  
  
//   fragColor = vec4(TRIMMED, 1.0);
// }


const labE: f64 = 216.0 / 24389.0;
const labK: f64 = 24389.0 / 27.0;
const rX: f64 = 0.9505;
const rY: f64 = 1.0000;
const rZ: f64 = 1.0891;
const sRGBiMatrix: [f64; 9] = [
    3.2404542, -1.5371385, -0.4985314,
    -0.9692660,  1.8760108,  0.0415560,
    0.0556434, -0.2040259,  1.0572252
];

/// Represents the LCH color spectrum, which is simply a lab color space represented in a polar coordinate system.                      
pub struct LCH {
    l: f64,
    c: f64,
    h: f64,
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
/// This struct represents a color via the CIELAB color space.
/// The LAB color space represents colors via three variables:
/// 
///   L -> Preceptual Lightness
/// 
///   a -> green-red axis, where negative values are more green, positive values are more red.
/// 
///   b -> blue-yellow axis, where negative values are more blue, and positive values are more yellow.
/// 
pub struct LAB {
    l: f64,
    a: f64,
    b: f64,
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

        let fY = (self.l + 16.0) / 116.0;
        let fX = (self.a / 500.0) + fY;
        let fZ = (self.b / 200.0) + fY;

        let fX = fX * fX * fX;
        let fY = fY * fY * fY;
        let fZ = fZ * fZ * fZ;

        let lX = (116.0 * fX - 16.0) / labK;
        let lY = (self.l / labK)     / labK;
        let lZ = (116.0 * fZ - 16.0) / labK;

        let dX  = if     fX <= labE {lX} else {fX};
        let dY  = if self.l <= labE {lY} else {fY};
        let dZ  = if     fZ <= labE {lZ} else {fZ};

        XYZ::new(rX * dX, rY * dY, rZ * dZ)
    }
}
/// A struct representing color via tristimulus values, which approximate
/// human vision.
pub struct XYZ {
    x: f64,
    y: f64,
    z: f64,
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
        let lR = ;
        let lG = ;
        let lB = ;

    }

}
///A representation of color via the three channels (or axis) red, green, and blue.
pub struct RGB {
    r: f64,
    g: f64,
    b: f64,
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

