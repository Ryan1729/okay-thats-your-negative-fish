#![allow(dead_code)]
//TODO either make pointy vs flat controllable or just support flat
//so we don't need this annotation
use std::f32::consts::PI;

const TAU_OVER_SIX: f32 = PI / 3f32;
const TAU_OVER_TWELEVE: f32 = PI / 6f32;


//TODO can these be const?
lazy_static!{
    static ref FLAT_UNIT_HEXAGON_XS: [f32 ; 7] =
        [f32::cos(TAU_OVER_SIX * 0f32),
         f32::cos(TAU_OVER_SIX * 1f32),
         f32::cos(TAU_OVER_SIX * 2f32),
         f32::cos(TAU_OVER_SIX * 3f32),
         f32::cos(TAU_OVER_SIX * 4f32),
         f32::cos(TAU_OVER_SIX * 5f32),
         f32::cos(TAU_OVER_SIX * 6f32)];

     static ref FLAT_UNIT_HEXAGON_YS: [f32 ; 7] =
         [f32::sin(TAU_OVER_SIX * 0f32),
          f32::sin(TAU_OVER_SIX * 1f32),
          f32::sin(TAU_OVER_SIX * 2f32),
          f32::sin(TAU_OVER_SIX * 3f32),
          f32::sin(TAU_OVER_SIX * 4f32),
          f32::sin(TAU_OVER_SIX * 5f32),
          f32::sin(TAU_OVER_SIX * 6f32)];

    pub static ref POINTY_UNIT_HEXAGON_XS: [f32 ; 7] =
        [f32::cos(TAU_OVER_TWELEVE * 1f32),
         f32::cos(TAU_OVER_TWELEVE * 3f32),
         f32::cos(TAU_OVER_TWELEVE * 5f32),
         f32::cos(TAU_OVER_TWELEVE * 7f32),
         f32::cos(TAU_OVER_TWELEVE * 9f32),
         f32::cos(TAU_OVER_TWELEVE * 11f32),
         f32::cos(TAU_OVER_TWELEVE * 13f32)];

    pub static ref POINTY_UNIT_HEXAGON_YS: [f32 ; 7] =
        [f32::sin(TAU_OVER_TWELEVE * 1f32),
         f32::sin(TAU_OVER_TWELEVE * 3f32),
         f32::sin(TAU_OVER_TWELEVE * 5f32),
         f32::sin(TAU_OVER_TWELEVE * 7f32),
         f32::sin(TAU_OVER_TWELEVE * 9f32),
         f32::sin(TAU_OVER_TWELEVE * 11f32),
         f32::sin(TAU_OVER_TWELEVE * 13f32)];
}

pub const PIECE_DIMENSIONS: (u16, u16) = (28, 52);
