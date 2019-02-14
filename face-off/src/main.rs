#![recursion_limit="128"]

#[cfg(feature = "asmjs")]
#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate lazy_static;

use std::time::Duration;

mod utils;
mod definitions;
mod affine;
mod imgtool;
mod histogram;
mod face_off;

use image::{ImageBuffer, Rgb, Rgba};
use rustface::Detector;

#[cfg(not(feature = "asmjs"))]
static MODEL_DATA:&[u8] = include_bytes!("../seeta_fd_frontal_v1.0.bin");

//.\emsdk.bat install latest
//.\emsdk.bat activate latest
// D:\emsdk1.38\emsdk\python\2.7.13.1_64bit\python-2.7.13.amd64
// D:\emsdk1.38\emsdk\clang\e1.38.24_64bit\binaryen\bin
// D:\emsdk1.38\emsdk\emscripten\1.38.24

//asmjs编译 在1.38的emscripten可编译通过

//如果编译完报错：TypeError: Right-hand side of 'instanceof' is not an object，说明是内存不够用！

#[cfg(not(feature = "asmjs"))]
fn run() {
    //初始化人脸识别器
    let mut detector1 = face_off::init_rustface(MODEL_DATA.to_vec()).unwrap();
    let famous:&[u8] = include_bytes!("../famous.jpg");
    let users:&[u8] = include_bytes!("../user.jpg");
    let famous = image::load_from_memory(famous).unwrap().to_rgba();
    let (fw,fh) = (famous.width(), famous.height());
    let users =  image::load_from_memory(users).unwrap().to_rgba();
    let (uw,uh) = (users.width(), users.height());
    let image = face_off::create(&mut detector1, (fw, fh, famous.into_raw()), (uw, uh, users.into_raw()), false);
    image.save("out.jpg").unwrap();
}

#[cfg(not(feature = "asmjs"))]
fn main() {
    println!("正常运行...");
    run();
}

static mut DETECTOR_PTR: *mut Box<Detector> = std::ptr::null_mut();
use std::sync::Mutex;

lazy_static!{
    static ref SRC_FAMOUS_IMAGE: Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>> = Mutex::new(None);//存储用户选择的底图
    static ref FF: Mutex<Option<face_off::Rectangle>> = Mutex::new(None);//存储底图的人脸位置
    static ref USER_FACE: Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>> = Mutex::new(None);//存储扣出的人脸图
}

#[cfg(feature = "asmjs")]
fn main() {
    use stdweb::{unstable::TryInto, web::ArrayBuffer, Value};
    use image::Pixel;

    stdweb::initialize();

    js!( getApp().face_off = {
        path:getApp().globalData.userDataPath+"binfile",
        showLoading: function(s){
            wx.showLoading({
                title: s,
                mask: true
            });
        },
        load: function(res){
            if(res){
                getApp().page.onFaceOffInit(res);
                return;
            }
            try{
                @{move |value:Value|{
                    match value{
                        Value::Reference(n) => {
                            let buffer:ArrayBuffer = n.try_into().unwrap();
                            let buffer = Vec::from(buffer);
                            match face_off::init_rustface(buffer){
                                Ok(d) =>{
                                    unsafe{ DETECTOR_PTR = Box::into_raw(Box::new(d)) };
                                    js!(getApp().page.onFaceOffInit());
                                }
                                Err(err) =>{
                                    js!(getApp().page.onFaceOffInit(@{format!("初始化失败：{:?}", err)}));
                                }
                            }
                        },
                        _ => {
                            js!(getApp().page.onFaceOffInit("初始化失败:数据格式不正确"));
                        }
                    }; 
                }}(getApp().globalData.FILE_DATA);
            }catch(e){
                console.log(e);
                getApp().page.onFaceOffInit("初始化失败:数据文件读取失败");
            }
        },
    }; );
    //初始化
    let init = || {
        js!(getApp().face_off.load(););
    };

    //旋转/缩放
    let resize = move |scale: f64, degeree:f64| -> Value{
        if !unsafe{ DETECTOR_PTR.is_null()}{
            let src_famous_image = SRC_FAMOUS_IMAGE.lock().unwrap();
            let ff = FF.lock().unwrap();
            let user_face = USER_FACE.lock().unwrap();
            //缩放, 旋转
            let image = face_off::replace_head(scale, degeree as f32 /180.0, src_famous_image.as_ref().unwrap(), ff.as_ref().unwrap(), user_face.as_ref().unwrap());

            // //生成jpg
            let mut file = vec![];
            match  image::jpeg::JPEGEncoder::new(&mut file).encode(&image, image.width(), image.height(), <Rgb<u8> as Pixel>::color_type()){
                Ok(()) =>{
                    file.into()
                }
                _ => Value::Null
            }
        }else{
            Value::Null
        }
    };

    //生成
    let create = move |base_image_data:Value, bw:f64, bh:f64, front_image_data:Value, fw:f64, fh:f64, fight: bool| -> Value{
        
        //清空
        *SRC_FAMOUS_IMAGE.lock().unwrap() = None;
        *FF.lock().unwrap() = None;
        *USER_FACE.lock().unwrap() = None;

        if !unsafe{ DETECTOR_PTR.is_null()}{
            let base_image_data:ArrayBuffer = base_image_data.try_into().unwrap();
            let base_image_data = Vec::from(base_image_data);
            let front_image_data:ArrayBuffer = front_image_data.try_into().unwrap();
            let front_image_data = Vec::from(front_image_data);
            let detector1 = unsafe { DETECTOR_PTR.as_mut() }.unwrap();
            // js!(console.log("create方法 底图("+@{bw}+","+@{bh}+")"));
            //js!(console.log("create方法 前景图("+@{fw}+","+@{fh}+")"));
            
            let mut src_famous_image = ImageBuffer::from_raw(bw as u32, bh as u32, base_image_data).unwrap();

            match face_off::clip_head(detector1, &mut src_famous_image, (fw as u32, fh as u32, front_image_data), fight){
                Ok((ff, user_face)) => {
                    //默认不缩放, 不旋转
                    let image = face_off::replace_head(1.0, 0.0, &src_famous_image, &ff, &user_face);

                    //存储
                    *SRC_FAMOUS_IMAGE.lock().unwrap() = Some(src_famous_image);            
                    *FF.lock().unwrap() = Some(ff);
                    *USER_FACE.lock().unwrap() = Some(user_face);

                    // //生成jpg
                    let mut file = vec![];
                    match  image::jpeg::JPEGEncoder::new(&mut file).encode(&image, image.width(), image.height(), <Rgb<u8> as Pixel>::color_type()){
                        Ok(()) =>{
                            file.into()
                        }
                        _ => Value::Null
                    }
                },
                Err(err) => err.into()
            }
        }else{
            Value::Null
        }
    };

    let convert_style = |base_image_data:Value, bw:f64, bh:f64, front_image_data:Value, fw:f64, fh:f64| -> Value{
        let base_image_data:ArrayBuffer = base_image_data.try_into().unwrap();
        let base_image_data = Vec::from(base_image_data);
        let front_image_data:ArrayBuffer = front_image_data.try_into().unwrap();
        let front_image_data = Vec::from(front_image_data);
        
        let src_famous_image = ImageBuffer::from_raw(bw as u32, bh as u32, base_image_data).unwrap();

        match face_off::convert_style(&src_famous_image, (fw as u32, fh as u32, front_image_data)){
            Some(image) => {
                //生成jpg
                let mut file = vec![];
                match  image::jpeg::JPEGEncoder::new(&mut file).encode(&image, image.width(), image.height(), <Rgba<u8> as Pixel>::color_type()){
                    Ok(()) =>{
                        file.into()
                    }
                    _ => Value::Null
                }
            }
            None => Value::Null
        }
    };

    js!{
        getApp().face_off.init = @{init};
        getApp().face_off.create = @{create};
        // getApp().face_off.load_image = @{load_image};
        getApp().face_off.convert_style = @{convert_style};
        getApp().face_off.resize = @{resize};
    };

    stdweb::event_loop();
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
