use super::detector::Detector;
use super::current_timestamp;
use crate::histogram;
// use crate::image_proc::{rotate_with_default, Interpolation};
use crate::imgtool;
use image::imageops::resize;
use image::{ConvertBuffer, GrayImage, ImageBuffer, Rgba, RgbaImage};
use image::imageops::FilterType;
use std::io;

//初始化人脸识别器
pub fn init_detector(cascade: Vec<u8>) -> Result<Detector, io::Error> {
    let mut detector = Detector::new(cascade);
    detector.set_minsize(100);
    detector.set_stridefactor(0.09);
    Ok(detector)
}

//人脸图片颜色转换
pub fn face_megic(
    famous_image_head: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    user_image_head: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    color_levels: i32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    //颜色量化
    let t = current_timestamp();
    let his = histogram::ColorQuantizationHistogram::new(color_levels as usize);
    println!("初始化histogram 耗时:{}ms", current_timestamp() - t);
    let t = current_timestamp();

    //-----------第一步，裁剪出名人图片的人脸区域(彩色)-----------------

    //颜色量化
    let famous_face = his.convert_image(&famous_image_head, 0, 0);
    let mut famous_face: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        famous_image_head.width(),
        famous_image_head.height(),
        famous_face,
    )
    .unwrap();
    println!(
        "底图人脸颜色量化 耗时:{}ms",
        current_timestamp() - t
    );
    let t = current_timestamp();

    //裁剪，边缘处理
    clip_ellipse(&mut famous_face);
    //#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    //js!(console.log("底图clip_ellipse 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    //---------------第二步，裁剪出用户的头像区域------------------
    //颜色量化
    let user_face = his.convert_image(&user_image_head, 0, 0);
    let mut user_face: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(user_image_head.width(), user_image_head.height(), user_face)
            .unwrap();
    println!("前景图颜色量化 耗时:{}ms", current_timestamp() - t);
    let t = current_timestamp();

    //裁剪
    clip_ellipse(&mut user_face);
    println!("前景图clip_ellipse 耗时:{}ms", current_timestamp() - t);
    let t = current_timestamp();
    ////js!(console.log("create>>007"));
    // user_face.save("out_user_head.png").unwrap(); //保存

    //--------------第三步，用户头像中和灰名人头像灰度级相同的颜色区域对应的坐标进行变色---

    imgtool::copy_hue(&famous_face, &mut user_face);

    println!("灰度级变色 耗时:{}ms", current_timestamp() - t);
    ////js!(console.log("create>>010"));
    // user_face_final.save("user_face_final.png").unwrap();
    ////js!(console.log("create>>011"));
    user_face
}

//替换拼接头像
// scale 缩放比例
// rotate 旋转角度
// src_famous_image -> 原图
// ff -> 原图头像位置
// user_face -> 裁剪出的用户头像图
// pub fn replace_head(
//     scale: f64,
//     rotate: f32,
//     translate: (f32, f32),
//     src_famous_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
//     ff: &Rectangle,
//     user_face: &ImageBuffer<Rgba<u8>, Vec<u8>>,
// ){
//     //----------- 缩放到和原图人脸区域同样大小 --------------
//     // let user_face_final = resize(
//     //     user_face,
//     //     (ff.width as f64 * scale) as u32,
//     //     (ff.height as f64 * scale) as u32,
//     //     image::FilterType::Nearest,
//     // );
//     let user_face_final = user_face;
//     let center = (
//         user_face_final.width() as f32 / 2f32,
//         user_face_final.height() as f32 / 2f32,
//     );
//     let user_face_final = rotate_with_default(
//         &user_face_final,
//         center,
//         rotate,
//         Rgba([0u8, 0u8, 0u8, 0u8]),
//         Interpolation::Nearest,
//     );

//     //js!(console.log("缩放 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

//     ////js!(console.log("create>>012"));
//     //--------------第四拼接到原图-----------
//     let (ow, oh) = (src_famous_image.width(), src_famous_image.height());

//     //超出部分需要进行移动
//     let x = ff.x + (ff.width as i32 - user_face_final.width() as i32) / 2;
//     let y = ff.y + (ff.height as i32 - user_face_final.height() as i32) / 2;

//     imgtool::draw_image(
//         x + (ow as f32 * translate.0) as i32,
//         y + (oh as f32 * translate.1) as i32,
//         src_famous_image,
//         &user_face_final);
// }

/// 检测有人脸区域
/// max_image_size: 图片宽高超过此值，紧缩缩小以后再识别
pub fn detect_faces(
    raw_image: &RgbaImage,
    detector: &mut Detector,
    max_image_size: u32,
) -> Vec<Rectangle> {
    let mut gray: GrayImage = raw_image.convert();

    if raw_image.width() > max_image_size || raw_image.height() > max_image_size {
        println!(
            "人脸识别：压缩图片 max_image_size={}",
            max_image_size
        );
        gray = resize_gray_image(&gray, max_image_size as f64);
    }
    let scale = raw_image.width() as f32 / gray.width() as f32;
    println!(
        "人脸识别：最终图片大小: {}x{}",
        gray.width(),
        gray.height()
    );

    let mut faces = vec![];
    for face in detector.detect_face(&gray) {
        //裁剪
        let width = face.radius * 2.0 * 0.82;
        let height = face.radius * 2.0 * 0.82;
        let x = face.x - width / 2.0;
        let y = face.y - height / 2.0 + height * 0.1;

        println!("找到人脸: {}x{} {}x{}", x, y, width, height);

        //恢复比例
        let width = width * scale;
        let height = height * scale;
        let x = x * scale;
        let y = y * scale;

        let mut rect = Rectangle::new(
            0f32.max(x) as i32,
            0f32.max(y) as i32,
            width as u32,
            height as u32,
        );
        //js!(console.log("检测到人脸：("+@{rect.x}+","+@{rect.y}+")"+"("+@{rect.width}+","+@{rect.height}+")"));
        //检测越界
        if (rect.x as u32 + rect.width) > raw_image.width() {
            rect.width = raw_image.width() - rect.x as u32;
        }
        if (rect.y as u32 + rect.height) > raw_image.height() {
            rect.height = raw_image.height() - rect.y as u32;
        }
        faces.push(rect);
    }
    faces
}

//压缩图片
// pub fn resize_image(
//     raw_image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
//     max_size: f64,
// ) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
//     let (width, height) = (raw_image.width() as f64, raw_image.height() as f64);
//     let mut new_width = width;
//     let mut new_height = height;
//     if width > max_size || height > max_size {
//         if width > max_size {
//             new_width = max_size;
//             new_height = (height / width) * max_size;
//         }
//         if new_height > max_size {
//             new_height = max_size;
//             new_width = (width / height) * max_size;
//         }
//         //#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
//         //js!(console.log("压缩大小:(" + @{new_width} + "," + @{new_height}+")"));
//     }
//     resize(
//         raw_image,
//         new_width as u32,
//         new_height as u32,
//         image::FilterType::Nearest,
//     )
// }

//压缩图片
pub fn resize_gray_image(raw_image: &GrayImage, max_size: f64) -> GrayImage {
    let (width, height) = (raw_image.width() as f64, raw_image.height() as f64);
    let mut new_width = width;
    let mut new_height = height;
    if width > max_size || height > max_size {
        if width > max_size {
            new_width = max_size;
            new_height = (height / width) * max_size;
        }
        if new_height > max_size {
            new_height = max_size;
            new_width = (width / height) * max_size;
        }
        //#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        //js!(console.log("压缩大小:(" + @{new_width} + "," + @{new_height}+")"));
    }
    resize(
        raw_image,
        new_width as u32,
        new_height as u32,
        FilterType::Nearest,
    )
}

//裁剪椭圆人脸区域(椭圆裁剪只是把椭圆以外的区域设置为透明，不改变图片大小)，裁边处理
pub fn clip_ellipse(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    //椭圆测试
    let (iw, ih) = (image.width(), image.height());
    let (cx, cy) = (iw as f64 / 2.0, ih as f64 * 0.45);
    let (a, b) = (cx * 0.9, cy * 1.37);
    //首先剪切椭圆区域
    for y in 0..ih {
        for x in 0..iw {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if (dx * dx) / (a * a) + (dy * dy) / (b * b) > 1.0 {
                //不在椭圆内的点全透明
                let pixel = &mut image[(x, y)];
                pixel[0] = 255;
                pixel[1] = 0;
                pixel[2] = 0;
                pixel[3] = 0;
            }
        }
    }

    //剪切额头部分(去掉刘海)
    let top = (ih as f64 * 0.1) as u32;
    for y in 0..top {
        for x in 0..iw {
            let pixel = &mut image[(x, y)];
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 0;
        }
    }

    //剪切两侧部分
    /*
    let side = (iw as f64 * 0.14) as usize;
    for row in user_head_data.chunks_mut(iw as usize * 4){
        for (x, col) in row.chunks_mut(4).enumerate() {
            if x<side || x>(iw as usize-side) {
                //不在椭圆内的点全透明
                col[0] = 0;
                col[1] = 0;
                col[2] = 0;
                col[3] = 0;
            }
        }
    }
    */

    //边缘虚化(边缘半透明)
    let border = (iw as f64 * 0.12) as i32;
    for row in image.chunks_mut(iw as usize * 4) {
        let mut pcount = 0i32;
        let mut t = false;
        for px in (0..row.len()).step_by(4) {
            //左边半透明
            if row[px + 3] > 0 && !t {
                //x越大透明度越低
                let per = pcount as f64 / border as f64;
                row[px + 3] = (per * 255.0) as u8;
                if pcount == border {
                    t = true;
                    pcount = border;
                }
                pcount += 1;
            }
            //右边半透明
            let stride = px + pcount as usize * 4;
            if t && row[stride + 3] == 0 && pcount != -1 {
                let mut per = pcount as f64 / border as f64;
                if per > 1.0 {
                    per = 1.0;
                }
                row[px + 3] = (per * 255.0) as u8;
                pcount -= 1;
            }
            if pcount < 0 {
                break;
            }
        }
    }

    //上边半透明
    let dt = (ih as f64 * 0.1) as u32;
    let bottom = top + dt;
    for y in top..bottom {
        for x in 0..iw {
            let pixel = &mut image[(x, y)];
            if pixel[3] > 0 {
                let per = 1.0 - (bottom - y) as f64 / dt as f64;
                let op = (per * 255.0) as u8;
                if op < pixel[3] {
                    pixel[3] = op;
                }
            }
        }
    }

    //下边半透明
    let top = (ih as f64 * 0.9) as u32;
    let border = ih - top; //下边框高度
    for y in top..ih {
        for x in 0..iw {
            let pixel = &mut image[(x, y)];
            if pixel[3] > 0 {
                let per = (ih - y) as f64 / border as f64;
                let op = (per * 255.0) as u8;
                if op < pixel[3] {
                    pixel[3] = op;
                }
            }
        }
    }

    //高于一定亮度的颜色变白
    // if fight_compute {
    //     for pixel in image.chunks_mut(4) {
    //         let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
    //             as i32;
    //         if gray > 110 {
    //             pixel[0] = 255;
    //             pixel[1] = 255;
    //             pixel[2] = 255;
    //             pixel[3] = 255;
    //         }
    //     }
    // }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}
