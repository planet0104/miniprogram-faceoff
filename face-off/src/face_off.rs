use image::{GrayImage, ConvertBuffer, GenericImage, ImageBuffer, Rgb, Rgba};
use image::imageops::resize;
// use image::imageops::colorops::huerotate;
use rustface::{Detector, ImageData};
use std::io;
use std::cmp::max;
use crate::histogram;
// use std::collections::HashMap;
// use std::io::prelude::*;
// use std::io::BufReader;
use crate::imgtool;
use crate::affine::*;

//初始化人脸识别器
pub fn init_rustface(model: Vec<u8>) -> Result<Box<Detector>, io::Error>{
    let model = rustface::model::read_model(model)?;
    let mut detector = rustface::create_detector_with_model(model);
    detector.set_min_face_size(20);
    detector.set_score_thresh(2.0);//默认:2.0, 阈值越小检测次数越多 0.95
    detector.set_pyramid_scale_factor(0.8);
    detector.set_slide_window_step(4, 4);
    Ok(detector)
}

//图片风格简单转换
pub fn convert_style(src_famous_image: &ImageBuffer<Rgba<u8>, Vec<u8>>, user_image_data: (u32, u32, Vec<u8>)) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>>{
    //颜色量化-底图
    let (fw, fh) = (src_famous_image.width(), src_famous_image.height());
    let his = histogram::ColorQuantizationHistogram::new(1024);
    let famous_image = his.convert_image(&src_famous_image, 0, 0);
    let famous_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(fw, fh, famous_image)?;

    //量化前景图
    let src_user_image:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(user_image_data.0, user_image_data.1, user_image_data.2)?;
    let (uw, uh) = (src_user_image.width(), src_user_image.height());
    let user_image = his.convert_image(&src_user_image, 0, 0);
    let mut user_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(uw, uh, user_image)?;

    imgtool::copy_hue(&famous_image, &mut user_image);

    Some(user_image)
}

//裁剪图片中的人脸
pub fn clip_head(detector:&mut Box<Detector>, src_famous_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, user_image_data: (u32, u32, Vec<u8>), fight_compute: bool) -> Result<(Rectangle, ImageBuffer<Rgba<u8>, Vec<u8>>), String>{
    //js!(console.log("face_off::create方法 前景图("+@{user_image_data.0}+","+@{user_image_data.1}+")"));

    //颜色量化
    //js!(getApp().t = Date.now());
    let his = histogram::ColorQuantizationHistogram::new(32);
    //js!(console.log("初始化histogram 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    //-----------第一步，裁剪出名人图片的人脸区域(彩色)-----------------
    // let mut src_famous_image = ImageBuffer::from_raw(famous_image_data.0, famous_image_data.1, famous_image_data.2).unwrap();
    //js!(console.log("加载底图 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    
    //js!(console.log("face_off::create方法>>底图image("+@{src_famous_image.width()}+","+@{src_famous_image.height()}+")"));
    
    let mut famous_faces = clip_faces(src_famous_image, detector);
    //js!(console.log("识别底图人脸 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    if famous_faces.len()==0{
        return Err(String::from("底图中没有检测到人脸(或人脸太小)，请重新拍照或选择图片"));
    }

    //颜色量化
    let (ff, i) = famous_faces.pop().unwrap();
    let famous_face = his.convert_image(&i, 0, 0);
    let mut famous_face: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(ff.width, ff.height, famous_face).unwrap();
    //js!(console.log("底图人脸颜色量化 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    
    //裁剪，边缘处理
    clip_ellipse(&mut famous_face, fight_compute);
    //js!(console.log("底图clip_ellipse 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    
    //---------------第二步，裁剪出用户的头像区域------------------
    let mut src_user_image = ImageBuffer::from_raw(user_image_data.0, user_image_data.1, user_image_data.2).unwrap();
    //js!(console.log("face_off::create方法>>前景图image("+@{src_user_image.width()}+","+@{src_user_image.height()}+")"));
    //js!(console.log("加载前景图 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    let mut user_faces = clip_faces(&mut src_user_image, detector);
    //js!(console.log("识别前景图人脸 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    if user_faces.len()==0{
        return Err(String::from("前景图中没有检测到人脸(或人脸太小)，请重新拍照或选择图片"));
    }
    //颜色量化
    let (uf, i) = user_faces.pop().unwrap();
    let user_face = his.convert_image(&i, 0, 0);
    let mut user_face: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(uf.width, uf.height, user_face).unwrap();
    //js!(console.log("前景图颜色量化 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    //裁剪
    clip_ellipse(&mut user_face, fight_compute);
    //js!(console.log("前景图clip_ellipse 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    //--------------第三步，用户头像中和灰名人头像灰度级相同的颜色区域对应的坐标进行变色---

    imgtool::copy_hue(&famous_face, &mut user_face);

    js!(console.log("灰度级变色 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    Ok((ff, user_face))
}

//替换拼接头像
// scale 缩放比例
// rotate 旋转角度
// src_famous_image -> 原图
// ff -> 原图头像位置
// user_face -> 裁剪出的用户头像图
pub fn replace_head(scale: f64, rotate: f32, src_famous_image: &ImageBuffer<Rgba<u8>, Vec<u8>>, ff: &Rectangle, user_face:&ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    //----------- 缩放到和原图人脸区域同样大小 --------------
    let user_face_final = resize(user_face, (ff.width as f64*scale) as u32, (ff.height as f64*scale) as u32, image::FilterType::Nearest);
    let center = (user_face_final.width() as f32 / 2f32, user_face_final.height() as f32 / 2f32);
    let user_face_final = rotate_with_default(&user_face_final, center, rotate, Rgba([0u8,0u8,0u8,0u8]), Interpolation::Nearest);

    //js!(console.log("缩放 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    //--------------第四拼接到原图-----------
    let (ow, oh) = (src_famous_image.width(), src_famous_image.height());
    let mut out_data = src_famous_image.clone().into_raw();//clone

    //超出部分需要进行移动
    let x = ff.x + (ff.width as i32-user_face_final.width() as i32)/2;
    let y = ff.y + (ff.height as i32-user_face_final.height() as i32)/2;

    let (x, y, width, height) = (x, y, user_face_final.width(), user_face_final.height());
    let user_face_data = user_face_final.into_raw();
    
    /* 颜色混合
        一般数学计算中，颜色取值是：R,G,B∈[0,255],A∈[0,1]
        所以对于一般的颜色混合有：Color(RGBA)=Color(R1,G1,B1,A1)+Color(R2,G2,B2,A2)

        标准的颜色混合算法如下：
        A=1−(1−α1)∗(1−α2)

        R=(α1*R1+(1−α1)*α2*R2)/A
        G=(α1*G1+(1−α1)*α2*G2)/A
        B=(α1*B1+(1−α1)*α2*B2)/A
    */
    let ostride = ow*4;
    let start = ostride*y as u32+x as u32*4;
    let end = start+ostride*height;
    let mut cy = 0;
    for p in (start..end).step_by(ostride as usize){
        for i in (0..width*4).step_by(4){
            let ui = (cy*width*4+i) as usize;
            let (r1, g1, b1, a1) = (user_face_data[ui] as f64, user_face_data[ui+1] as f64, user_face_data[ui+2] as f64, user_face_data[ui+3] as f64/255.0);
            let fi = (p+i) as usize;
            let (r2, g2, b2, a2) = (out_data[fi] as f64, out_data[fi+1] as f64, out_data[fi+2] as f64, out_data[fi+3] as f64/255.0);

            let a = 1.0-(1.0-a1)*(1.0-a2);
            let r = (a1*r1+(1.0-a1)*a2*r2)/a;
            let g = (a1*g1+(1.0-a1)*a2*g2)/a;
            let b = (a1*b1+(1.0-a1)*a2*b2)/a;

            out_data[fi] = r as u8;
            out_data[fi+1] = g as u8;
            out_data[fi+2] = b as u8;
            out_data[fi+3] = (a*255.0) as u8;
        }
        cy += 1;
    }

    //js!(console.log("拼接 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());
    
    //转rgb
    let mut rgb_data = vec![];
    for pixel in out_data.chunks(4){
        rgb_data.extend_from_slice(&pixel[..3]);
    }

    //js!(console.log("转rgb 耗时:"+(Date.now()-getApp().t)+"ms"); getApp().t = Date.now());

    ImageBuffer::from_raw(ow, oh, rgb_data).unwrap()
}

//检测并输出所有人脸子图
fn clip_faces(
    raw_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    detector: &mut Box<Detector>,
) -> Vec<(Rectangle, ImageBuffer<Rgba<u8>, Vec<u8>>)> {

    //缩放图片提高识别速度
    let max_size = 230.0;
    let (width, height) = (raw_image.width() as f64, raw_image.height() as f64);
    let mut new_width = width;
    let mut new_height = height;
    if width>max_size || height>max_size{
        js!(getApp().face_off.showLoading("压缩图片"));
        js!(console.log("原始大小:(" + @{width} + "," + @{height}+")"));
        if width > max_size {
            new_width = max_size;
            new_height = (height / width) * max_size;
        }
        if new_height > max_size {
            new_height = max_size;
            new_width = (width / height) * max_size;
        }
        js!(console.log("压缩大小:(" + @{new_width} + "," + @{new_height}+")"));
    }
    let image = resize(raw_image, new_width as u32, new_height as u32, image::FilterType::Nearest);

    let gray:GrayImage = image.convert();

    let width_scale = width as f64/gray.width() as f64;
    let height_scale = height as f64/gray.height() as f64;

    js!(console.log("clip_faces>>图片大小("+@{gray.width()}+","+@{gray.height()}+")"));
    let mut image_data = ImageData::new(gray.as_ptr(), gray.width(), gray.height());
    let mut faces = vec![];
    // js!(console.log("clip_faces>>002"));
    js!(getApp().face_off.showLoading("识别人脸"));
    for face in detector.detect(&mut image_data) {
        js!(console.log("clip_faces>>找到人脸"));
        // println!("找到人脸: {:?}", face);
        /*
        bbox: Rectangle,
    roll: f64,
    pitch: f64,
    yaw: f64,
    score: f64,
         */
        //js!(console.log("找到人脸 bbox{x: "+@{face.bbox().x()}+", y: "+@{face.bbox().y()}+", width: "+@{face.bbox().width()}+", height: "+@{face.bbox().height()}+"}"+" roll="+@{face.roll}+", pitch="+@{face.pitch}+", yaw="+@{face.yaw}+", score="+@{face.score}));
        //裁剪
        let face_bbox = face.bbox();

        let rect = Rectangle::new(
            (max(0, face_bbox.x()) as f64 * width_scale) as i32,
            (max(0, face_bbox.y()) as f64 * height_scale) as i32,
            (face_bbox.width() as f64 * width_scale) as u32,
            (face_bbox.height() as f64 * height_scale) as u32,
        );
        //js!(console.log("检测到人脸：("+@{rect.x}+","+@{rect.y}+")"));
        faces.push((
            rect.clone(),
            raw_image
                .sub_image(rect.x as u32, rect.y as u32, rect.width, rect.height)
                .to_image(),
        ));
    }
    faces
}


//裁剪椭圆人脸区域(椭圆裁剪只是把椭圆以外的区域设置为透明，不改变图片大小)，裁边处理
//fight_compute: 是否进行斗图处理
fn clip_ellipse(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, fight_compute: bool){
    //椭圆测试
    let (iw, ih) = (image.width(), image.height());
    let (cx, cy) = (iw as f64 / 2.0, ih as f64 * 0.45);
    let (a, b) = (cx * 0.8, cy*1.4);
    //首先剪切椭圆区域
    for y in 0..ih{
        for x in 0..iw{
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
    let top = (ih as f64*0.15) as u32;
    for y in 0..top{
        for x in 0..iw{
            let pixel = &mut image[(x, y)];
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 0;
        }
    }

    //边缘虚化(边缘半透明)
    let border = (iw as f64*0.12) as i32;
    for row in image.chunks_mut(iw as usize * 4){
        let mut pcount = 0i32;
        let mut t = false;
        for px in (0..row.len()).step_by(4){
            //左边半透明
            if row[px+3]>0 && !t{
                //x越大透明度越低
                let per = pcount as f64/border as f64;
                row[px+3] = (per*255.0) as u8;
                if pcount==border{
                    t = true;
                    pcount = border;
                }
                pcount += 1;
            }
            //右边半透明
            let stride = px+pcount as usize*4;
            if t && row[stride+3]==0 && pcount!=-1{
                let mut per = pcount as f64/border as f64;
                if per>1.0 {
                    per = 1.0;
                }
                row[px+3] = (per*255.0) as u8;
                pcount -= 1;
            }
            if pcount<0{
                break;
            }
        }
    }

    //上边半透明
    let dt = (ih as f64*0.1) as u32;
    let bottom = top+dt;
    for y in top..bottom{
        for x in 0..iw{
            let pixel = &mut image[(x, y)];
            if pixel[3]>0{
                let per = 1.0-(bottom-y) as f64/dt as f64;
                let op = (per*255.0) as u8;
                if op<pixel[3]{
                    pixel[3] = op;
                }
            }
        }
    }

    //下边半透明
    let top = (ih as f64*0.9) as u32;
    let border = ih-top;//下边框高度
    for y in top..ih{
        for x in 0..iw{
            let pixel = &mut image[(x, y)];
            if pixel[3]>0{
                let per = (ih - y) as f64/border as f64;
                let op = (per*255.0) as u8;
                if op<pixel[3]{
                    pixel[3] = op;
                }
            }
        }
    }

    //高于一定亮度的颜色变白
    if fight_compute{
        for pixel in image.chunks_mut(4){
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                    as i32;
            if gray>110{
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
        }
    }
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