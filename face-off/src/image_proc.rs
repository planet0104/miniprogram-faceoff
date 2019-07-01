//! Functions for affine transformations of images.

// use crate::definitions::{Clamp, Image};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel};
// use image::{Luma, Rgb, Rgba};
type Image<P> = ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>;

use std::{f32, u16, u8};
pub trait Clamp<T> {
    /// Clamp `x` to a valid value for this type.
    fn clamp(x: T) -> Self;
}
/// Creates an implementation of Clamp<From> for type To.
macro_rules! implement_clamp {
    ($from:ty, $to:ty, $min:expr, $max:expr, $min_from:expr, $max_from:expr) => {
        impl Clamp<$from> for $to {
            fn clamp(x: $from) -> $to {
                if x < $max_from as $from {
                    if x > $min_from as $from {
                        x as $to
                    } else {
                        $min
                    }
                } else {
                    $max
                }
            }
        }
    };
}

implement_clamp!(f32, u8, u8::MIN, u8::MAX, u8::MIN as f32, u8::MAX as f32);
implement_clamp!(
    f32,
    u16,
    u16::MIN,
    u16::MAX,
    u16::MIN as f32,
    u16::MAX as f32
);

/// How to handle pixels whose pre-image lies between input pixels.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Interpolation {
    /// Choose the nearest pixel to the pre-image of the
    /// output pixel.
    Nearest,
    /// Bilinearly interpolate between the four pixels
    /// closest to the pre-image of the output pixel.
    Bilinear,
}

/// Rotate an image clockwise about provided center by theta radians.
/// The output image has the same dimensions as the input. Output pixels
/// whose pre-image lies outside the input image are set to default.
pub fn rotate_with_default<P>(
    image: &Image<P>,
    center: (f32, f32),
    theta: f32,
    default: P,
    interpolation: Interpolation,
) -> Image<P>
where
    P: Pixel + 'static,
    <P as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    match interpolation {
        Interpolation::Nearest => rotate_nearest(image, center, theta, default),
        Interpolation::Bilinear => rotate_bilinear(image, center, theta, default),
    }
}

fn rotate_nearest<P>(image: &Image<P>, center: (f32, f32), theta: f32, default: P) -> Image<P>
where
    P: Pixel + 'static,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let center_x = center.0;
    let center_y = center.1;

    for y in 0..height {
        let dy = y as f32 - center_y;
        let mut px = center_x + sin_theta * dy - cos_theta * center_x;
        let mut py = center_y + cos_theta * dy + sin_theta * center_x;

        for x in 0..width {
            unsafe {
                let pix = nearest(image, px, py, default);
                out.unsafe_put_pixel(x, y, pix);
            }

            px += cos_theta;
            py -= sin_theta;
        }
    }

    out
}

fn rotate_bilinear<P>(image: &Image<P>, center: (f32, f32), theta: f32, default: P) -> Image<P>
where
    P: Pixel + 'static,
    <P as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);

    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let center_x = center.0;
    let center_y = center.1;

    for y in 0..height {
        let dy = y as f32 - center_y;
        let mut px = center_x + sin_theta * dy - cos_theta * center_x;
        let mut py = center_y + cos_theta * dy + sin_theta * center_x;

        for x in 0..width {
            let pix = interpolate(image, px, py, default);
            unsafe {
                out.unsafe_put_pixel(x, y, pix);
            }

            px += cos_theta;
            py -= sin_theta;
        }
    }

    out
}

fn interpolate<P>(image: &Image<P>, x: f32, y: f32, default: P) -> P
where
    P: Pixel + 'static,
    <P as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    let left = x.floor();
    let right = left + 1f32;
    let top = y.floor();
    let bottom = top + 1f32;

    let right_weight = x - left;
    let bottom_weight = y - top;

    // default if out of bound
    let (width, height) = image.dimensions();
    if left < 0f32 || right >= width as f32 || top < 0f32 || bottom >= height as f32 {
        default
    } else {
        let (tl, tr, bl, br) = unsafe {
            (
                image.unsafe_get_pixel(left as u32, top as u32),
                image.unsafe_get_pixel(right as u32, top as u32),
                image.unsafe_get_pixel(left as u32, bottom as u32),
                image.unsafe_get_pixel(right as u32, bottom as u32),
            )
        };
        blend(tl, tr, bl, br, right_weight, bottom_weight)
    }
}

fn nearest<P: Pixel + 'static>(image: &Image<P>, x: f32, y: f32, default: P) -> P {
    let rx = x.round();
    let ry = y.round();

    // default if out of bound
    let (width, height) = image.dimensions();
    if rx < 0f32 || rx >= width as f32 || ry < 0f32 || ry >= height as f32 {
        default
    } else {
        unsafe { image.unsafe_get_pixel(rx as u32, ry as u32) }
    }
}

fn blend<P>(
    top_left: P,
    top_right: P,
    bottom_left: P,
    bottom_right: P,
    right_weight: f32,
    bottom_weight: f32,
) -> P
where
    P: Pixel,
    P::Subpixel: Into<f32> + Clamp<f32>,
{
    let top = top_left.map2(&top_right, |u, v| {
        P::Subpixel::clamp((1f32 - right_weight) * u.into() + right_weight * v.into())
    });

    let bottom = bottom_left.map2(&bottom_right, |u, v| {
        P::Subpixel::clamp((1f32 - right_weight) * u.into() + right_weight * v.into())
    });

    top.map2(&bottom, |u, v| {
        P::Subpixel::clamp((1f32 - bottom_weight) * u.into() + bottom_weight * v.into())
    })
}

// Helper for a conversion that we know can't fail.
// pub fn cast<T, U>(x: T) -> U
// where
//     T: Into<U>,
// {
//     match x.try_into() {
//         Ok(y) => y,
//         Err(_) => panic!("Failed to convert"),
//     }
// }
