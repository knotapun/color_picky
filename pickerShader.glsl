const float labE = 216.0/24389.0;
const float labK = 24389.0/27.0;
const float pi = 3.14159;
const vec3 rXYZ = vec3 (0.9505, 1.0000, 1.0891);
const mat3 sRGBiMatrix = mat3 (
   3.2404542, -1.5371385, -0.4985314,
  -0.9692660,  1.8760108,  0.0415560,
   0.0556434, -0.2040259,  1.0572252
);

#define NEWTON_ITER 2
#define HALLEY_ITER 2

//Stolen from Scholarius: https://www.shadertoy.com/view/wts3RX 
//Turns out, cube roots are kind of annoying to calculate.
float cbrt( float x )
{
	float y = sign(x) * uintBitsToFloat( floatBitsToUint( abs(x) ) / 3u + 0x2a514067u );

	for( int i = 0; i < NEWTON_ITER; ++i )
    	y = ( 2. * y + x / ( y * y ) ) * .333333333;

    for( int i = 0; i < HALLEY_ITER; ++i )
    {
    	float y3 = y * y * y;
        y *= ( y3 + 2. * x ) / ( 2. * y3 + x );
    }
    
    return y;
}

vec3 LCHtoLAB(vec3 LCH) {
    float rads = radians(LCH.z);
    vec3 LAB = vec3(LCH[0], cos(rads) * LCH[1],sin(rads) * LCH[1]);
    return LAB;
}

vec3 LABtoXYZ(vec3 LAB) {

    float fY = (LAB[0] + 16.0) / 116.0;
    
    vec3 fXYZ = vec3( 
         LAB[1]/500.0 + fY,
         fY,
        -LAB[2]/200.0 + fY
    );
    
    fXYZ = fXYZ * fXYZ * fXYZ;
    
    
    vec3 lowXYZ = vec3(
        116.0 * fXYZ[0] - 16.0,
        LAB[0]/labK,
        116.0 * fXYZ[2] - 16.0
    ) / vec3(labK); 
    
    vec3 dXYZ = vec3(
    ((fXYZ[0] <= labE)          ? lowXYZ[0] : fXYZ[0]),
    (( LAB[0] <= (labK * labE)) ? lowXYZ[1] : fXYZ[1]),
    ((fXYZ[2] <= labE)          ? lowXYZ[2] : fXYZ[2])
    );
    return rXYZ * dXYZ;
}

vec3 XYZtoRGB(vec3 XYZ) {
    vec3 lRGB = XYZ * sRGBiMatrix;
    vec3 lowRGB = lRGB * labK/100.0;
    vec3 highRGB = (vec3(
        cbrt(lRGB[0]),
        cbrt(lRGB[1]),
        cbrt(lRGB[2])
    ) * 1.16) - vec3(0.16);
    vec3 RGB = vec3(
    (lRGB[0] <= labE) ? lowRGB[0] : highRGB[0],
    (lRGB[1] <= labE) ? lowRGB[1] : highRGB[1], 
    (lRGB[2] <= labE) ? lowRGB[2] : highRGB[2]
  );
  return RGB;
}

vec3 coordsToLCH(vec2 fragCoord, vec4 iMouse, vec3 iResolution) {
  vec3 coords = vec3(
       iMouse.x / iResolution.x,
    fragCoord.y / iResolution.y,
    fragCoord.x / iResolution.x
  );
  vec3 scaling = vec3(100.0, 128.0, 360.0);
  return coords * scaling;
}

vec3 trimByValue(vec3 RGB, vec2 fragCoord) {
  if ( RGB.r < 0.0 || RGB.r > 1.0 || RGB.g < 0.0 || RGB.g > 1.0 || RGB.b < 0.0 || RGB.b > 1.0) {
    bool horizontalMod = mod(fragCoord.x, 20.0) > 10.0;
    bool verticalMod = mod(fragCoord.y, 20.0) > 10.0;
  
      if((horizontalMod && !verticalMod) || (!horizontalMod && verticalMod)) {
        RGB = vec3(0.5);
      } else {
        RGB = vec3(0.7);
    }
  }
  return RGB;
}

/*  WIP 
    attempts to draw circles of the color under the mouse via shader.
    Will need to be called early on, and will work by manipulating what
    calls further down the line think the actual coordinates of the pixels within the
    cicle are the coordinates of the mouse itself, meaning their color will just be 
    calculated to be the same as that pixel.
*/
vec3 addMouseHighlight(vec3 RGB, vec2 fragCoord, vec4 iMouse) {
    vec2 mouseLocation =    iMouse.xy / iResolution.xy;
    vec2 pixelLocation = fragCoord.xy / iResolution.xy;
    vec2 dist = mouseLocation - pixelLocation;
    float dis = sqrt(dist.x * dist.x + dist.y * dist.y);
    if(dis > 10.0){
        RGB = vec3(0.5);
    }
    return RGB;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
  
  vec3 LCH = coordsToLCH(fragCoord, iMouse, iResolution);
    
  vec3 LAB = LCHtoLAB(LCH);
   
  vec3 XYZ = LABtoXYZ(LAB);
  
  vec3 RGB = XYZtoRGB(XYZ);
  
  vec3 TRIMMED = trimByValue(RGB, fragCoord);
  
  
  fragColor = vec4(TRIMMED, 1.0);
}