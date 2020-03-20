use std::cell::RefCell;
use std::time::Duration;
mod detector;
mod face_off;
mod histogram;
mod image_proc;
mod imgtool;
use crate::image_proc::{rotate_with_default, Interpolation};
use image::{ColorType, ImageBuffer, Rgba};
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_float, c_int};
use image::imageops::FilterType;

//.\emsdk.bat install latest
//.\emsdk.bat activate latest
// D:\emsdk1.38\emsdk\python\2.7.13.1_64bit\python-2.7.13.amd64
// D:\emsdk1.38\emsdk\clang\e1.38.24_64bit\binaryen\bin
// D:\emsdk1.38\emsdk\emscripten\1.38.24

//asmjs编译 在1.38的emscripten可编译通过

//如果编译完报错：TypeError: Right-hand side of 'instanceof' is not an object，说明是内存不够用！

// extern "C" {
//     pub fn emscripten_run_script(msg: *const c_char);
//     pub fn emscripten_run_script_string(script: *const c_char) -> *mut c_char;
// }

extern "C" {
    fn _current_timestamp() -> c_double;
    fn js_show_loading(msg: *const c_char);
    fn js_set_module();
    fn js_show_modal(msg: *const c_char);
    fn js_set_app_field(key: *const c_char, value: *const c_char);
    fn js_set_app_field_json(key: *const c_char, value: *const c_char);
    fn js_delete_app_field(key: *const c_char);
    fn js_get_cascade_file_data() -> *mut c_char;
    fn js_get_cascade_file_data_size() -> c_int;
}
// pub fn run_script(script:&str){
//     let url = CString::new(script).unwrap();
//     let ptr = url.as_ptr();
//     unsafe{
//         emscripten_run_script(ptr);
//     }
// }

//获取cascade文件的数据
// *mut c_char 表示需要在rust代码中进行内存释放
pub fn get_cascade_file_data() -> Vec<u8> {
    unsafe {
        let ptr = js_get_cascade_file_data();
        let len = js_get_cascade_file_data_size();
        Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize)
    }
}

pub fn current_timestamp() -> u64 {
    let t = unsafe { _current_timestamp() };
    t as u64
}

pub fn show_loading(msg: &str) {
    let url = CString::new(msg).unwrap();
    let ptr = url.as_ptr();
    unsafe { js_show_loading(ptr) };
}

pub fn show_modal(msg: &str) {
    let url = CString::new(msg).unwrap();
    let ptr = url.as_ptr();
    unsafe { js_show_modal(ptr) };
}

pub fn delete_app_field(key: &str) {
    let url = CString::new(key).unwrap();
    let ptr = url.as_ptr();
    unsafe { js_delete_app_field(ptr) };
}

pub fn set_app_field(key: &str, value: &str) {
    let url = CString::new(key).unwrap();
    let key_ptr = url.as_ptr();
    let url = CString::new(value).unwrap();
    let value_ptr = url.as_ptr();
    unsafe { js_set_app_field(key_ptr, value_ptr) };
}

pub fn set_app_field_json(key: &str, value: &str) {
    let url = CString::new(key).unwrap();
    let key_ptr = url.as_ptr();
    let url = CString::new(value).unwrap();
    let value_ptr = url.as_ptr();
    unsafe { js_set_app_field_json(key_ptr, value_ptr) };
}

#[cfg(windows)]
fn run() {
    //给定两个图片路径，指定参数， 生成照片

    //初始化人脸识别器
    let mut detector = face_off::init_rustface(MODEL_DATA.to_vec()).unwrap();
    let famous: &[u8] = include_bytes!("../famous.jpg");
    let users: &[u8] = include_bytes!("../th.jpg");
    let mut famous = image::load_from_memory(famous).unwrap().to_rgba();

    let (fw, fh) = (famous.width(), famous.height());
    let users = image::load_from_memory(users).unwrap().to_rgba();
    let (uw, uh) = (users.width(), users.height());

    let (ff, user_face) = face_off::clip_head(
        &mut detector,
        &mut famous,
        (uw as u32, uh as u32, users.into_raw()),
        false,
    )
    .unwrap();
    face_off::replace_head(1.05, 0.0, (0.018, 0.005), &mut famous, &ff, &user_face);

    let text_image = image::open("text.png").unwrap().to_rgba();
    //绘制文字
    imgtool::draw_image(50, 50, &mut famous, &text_image);

    famous.save("out.jpg").unwrap();
}

#[cfg(windows)]
fn main() {
    println!("正常运行...");
    run();
}

#[cfg(windows)]
pub fn log<T: std::fmt::Debug>(s: T) {
    println!("{:?}", s);
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
struct MyContext {
    detector: Option<detector::Detector>,
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
thread_local! {
    static CONTEXT: RefCell<MyContext> = RefCell::new(MyContext{
        detector: None
    });
}

//https://emscripten.org/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html#calling-javascript-from-c-c

/// 参数： 底图数据、底图人脸数据、前景图人脸数据
/// 替换前景图中的头像并生成最终图片数据
///
/// (1)选择新的前景图或者背景图时调用
///
/// (2)用户拖动（旋转/缩放）按钮时调用
///
/// 返回： getApp().replace_head_result = 图片数据 指针
///
// #[no_mangle]
// pub fn replace_head(
//     base_image_buffer:*mut u8,
//     base_image_buffer_len: c_int,
//     base_image_width: c_int,
//     base_image_height: c_int,

//     front_head_image_buffer:*mut u8,
//     front_head_image_buffer_len: c_int,
//     front_head_image_width: c_int,
//     front_head_image_height: c_int,

//     base_head_image_buffer:*mut u8,
//     base_head_image_buffer_len: c_int,
//     base_head_image_width: c_int,
//     base_head_image_height: c_int,

//     face_rect_x: c_int,
//     face_rect_y: c_int,
//     face_rect_width: c_int,
//     face_rect_height: c_int,

//     scale: c_double,
//     degree: c_double,
//     translate_x: c_int,
//     translate_y: c_int,
//     color_levels: c_int,
//     ){

//     let base_image_data:Vec<u8> = unsafe{ Vec::from_raw_parts(base_image_buffer, base_image_buffer_len as usize, base_image_buffer_len as usize) };
//     let front_head_image_data:Vec<u8> = unsafe{ Vec::from_raw_parts(front_head_image_buffer, front_head_image_buffer_len as usize, front_head_image_buffer_len as usize) };
//     let base_head_image_data:Vec<u8> = unsafe{ Vec::from_raw_parts(base_head_image_buffer, base_head_image_buffer_len as usize, base_head_image_buffer_len as usize) };

//     let (bw, bh) = {
//         (base_image_width as u32, base_image_height as u32)
//     };
//     let base_head_image_width = base_head_image_width as u32;
//     let base_head_image_height = base_head_image_height as u32;

//     let front_head_image_width = front_head_image_width as u32;
//     let front_head_image_height = front_head_image_height as u32;

//     // println!("replace_head() 底图大小:{}x{}", bw, bh);
//     let base_head_image:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(base_head_image_width, base_head_image_height, base_head_image_data).unwrap();
//     let front_head_image:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(front_head_image_width, front_head_image_height, front_head_image_data).unwrap();
//     let mut src_famous_image:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(bw, bh, base_image_data).unwrap();

//     let user_face = face_off::face_megic(&base_head_image, &front_head_image, color_levels);
//     let face_rect = face_off::Rectangle::new(
//         face_rect_x,
//         face_rect_y,
//         face_rect_width as u32,
//         face_rect_height as u32);

//     //替换头像(指定缩放、旋转)
//     face_off::replace_head(
//         scale,
//         degree as f32 /180.0,
//         (translate_x as f32, translate_y as f32),
//         &mut src_famous_image,
//         &face_rect, &user_face);
//     // println!("rust>>replace_head:头像替换完成..");
//     let (w,h) = (src_famous_image.width(), src_famous_image.height());
//     let mut image_data = src_famous_image.into_raw();

//     let len = image_data.len();
//     let ptr = image_data.as_mut_ptr() as *mut c_char;
//     std::mem::forget(image_data);
//     set_app_field_json("replace_head_result",
//         &format!(r#"{{
//         "imageDataLen": {},
//         "imageDataPtr": {},
//         "width": {},
//         "height": {}
//     }}"#, len, ptr as i64, w, h));
// }

/// 参数： 底图人脸数据、前景图人脸数据
/// 根据底图人脸数据，变换前景人脸图像
///
/// (1)选择新的前景图或者背景图时调用
///
/// (2)用户选择色阶时使用
///
/// 返回： getApp().face_megic_result = 图片数据 指针
///
#[no_mangle]
pub fn face_megic(
    front_head_image_buffer: *mut u8,
    front_head_image_buffer_len: c_int,
    front_head_image_width: c_int,
    front_head_image_height: c_int,

    base_head_image_buffer: *mut u8,
    base_head_image_buffer_len: c_int,
    base_head_image_width: c_int,
    base_head_image_height: c_int,
    color_levels: c_int,
) {
    let front_head_image_data: Vec<u8> = unsafe {
        Vec::from_raw_parts(
            front_head_image_buffer,
            front_head_image_buffer_len as usize,
            front_head_image_buffer_len as usize,
        )
    };
    let base_head_image_data: Vec<u8> = unsafe {
        Vec::from_raw_parts(
            base_head_image_buffer,
            base_head_image_buffer_len as usize,
            base_head_image_buffer_len as usize,
        )
    };

    let base_head_image_width = base_head_image_width as u32;
    let base_head_image_height = base_head_image_height as u32;

    let front_head_image_width = front_head_image_width as u32;
    let front_head_image_height = front_head_image_height as u32;

    // println!("replace_head() 底图大小:{}x{}", bw, bh);
    let base_head_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        base_head_image_width,
        base_head_image_height,
        base_head_image_data,
    )
    .unwrap();
    let front_head_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        front_head_image_width,
        front_head_image_height,
        front_head_image_data,
    )
    .unwrap();

    let user_face = face_off::face_megic(&base_head_image, &front_head_image, color_levels);

    let (w, h) = (user_face.width(), user_face.height());
    let mut image_data = user_face.into_raw();

    let len = image_data.len();
    let ptr = image_data.as_mut_ptr() as *mut c_char;
    std::mem::forget(image_data);
    set_app_field_json(
        "face_megic_result",
        &format!(
            r#"{{
        "imageDataLen": {},
        "imageDataPtr": {},
        "width": {},
        "height": {}
    }}"#,
            len, ptr as i64, w, h
        ),
    );
}

/// 图片数据生成jpeg
///
/// return create_jpeg_result = jpeg data ptr
#[no_mangle]
pub fn create_jpeg(
    src_image_buffer: *mut u8,
    src_image_buffer_len: c_int,
    src_image_width: c_int,
    src_image_height: c_int,
    output_scale: c_double,
) {
    let src_image_data: Vec<u8> = unsafe {
        Vec::from_raw_parts(
            src_image_buffer,
            src_image_buffer_len as usize,
            src_image_buffer_len as usize,
        )
    };

    let image_data: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        src_image_width as u32,
        src_image_height as u32,
        src_image_data,
    )
    .unwrap();
    // println!("create_jpeg output_scale={}", output_scale);
    //缩放
    let nwidth = src_image_width as f64 * output_scale;
    let nheight = src_image_height as f64 * output_scale;
    let image_data = image::imageops::resize(
        &image_data,
        nwidth as u32,
        nheight as u32,
        FilterType::Nearest,
    );

    let mut file = vec![];
    match image::jpeg::JPEGEncoder::new(&mut file).encode(
        &image_data,
        image_data.width(),
        image_data.height(),
        ColorType::Rgba8
    ) {
        Ok(()) => {
            let len = file.len();
            let ptr = file.as_mut_ptr() as *mut c_char;
            std::mem::forget(file);
            set_app_field_json(
                "create_jpeg_result",
                &format!(
                    r#"{{
                    "imageDataLen": {},
                    "imageDataPtr": {}
                }}"#,
                    len, ptr as i64
                ),
            );
        }
        _ => {
            delete_app_field("create_jpeg_result");
        }
    }
}

/// 图片数据生成png
///
/// return create_png_result = png data ptr
#[no_mangle]
pub fn create_png(
    src_image_buffer: *mut u8,
    src_image_buffer_len: c_int,
    src_image_width: c_int,
    src_image_height: c_int,
) {
    let src_image_data: Vec<u8> = unsafe {
        Vec::from_raw_parts(
            src_image_buffer,
            src_image_buffer_len as usize,
            src_image_buffer_len as usize,
        )
    };

    let image_data: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        src_image_width as u32,
        src_image_height as u32,
        src_image_data,
    )
    .unwrap();

    let mut file = vec![];

    match image::png::PNGEncoder::new(&mut file).encode(
        &image_data,
        image_data.width(),
        image_data.height(),
        ColorType::Rgba8,
    ) {
        Ok(()) => {
            let len = file.len();
            let ptr = file.as_mut_ptr() as *mut c_char;
            std::mem::forget(file);
            set_app_field_json(
                "create_png_result",
                &format!(
                    r#"{{
                    "imageDataLen": {},
                    "imageDataPtr": {}
                }}"#,
                    len, ptr as i64
                ),
            );
        }
        _ => {
            delete_app_field("create_png_result");
        }
    }
}

/// 绘制子图片
///
/// return draw_image_result = file data ptr
// #[no_mangle]
// pub fn draw_image(
//     base_image_buffer: *mut u8,
//     base_image_buffer_len: c_int,
//     base_image_width: c_int,
//     base_image_height: c_int,

//     image_buffer: *mut u8,
//     image_buffer_len: c_int,
//     image_width: c_int,
//     image_height: c_int,

//     x: c_int,
//     y: c_int,
//     width: c_int,
//     height: c_int,
//     rotate_degree: c_float,
// ) {
//     println!("draw_image>>开始..");
//     let base_image_data: Vec<u8> = unsafe {
//         Vec::from_raw_parts(
//             base_image_buffer,
//             base_image_buffer_len as usize,
//             base_image_buffer_len as usize,
//         )
//     };
//     let image_data: Vec<u8> = unsafe {
//         Vec::from_raw_parts(
//             image_buffer,
//             image_buffer_len as usize,
//             image_buffer_len as usize,
//         )
//     };

//     // println!("draw_image base_image size={} {}x{}", base_image_data.len(), base_image_width, base_image_height);
//     // println!("draw_image image size={} {}x{}", image_data.len(), image_width, image_height);

//     let mut src_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
//         base_image_width as u32,
//         base_image_height as u32,
//         base_image_data,
//     )
//     .unwrap();

//     let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
//         ImageBuffer::from_raw(image_width as u32, image_height as u32, image_data).unwrap();
//     if width != image_width || height != image_height {
//         println!("draw_image>>缩放..");
//         show_loading("缩放图片");
//         image = image::imageops::resize(
//             &image,
//             width as u32,
//             height as u32,
//             image::FilterType::Nearest,
//         );
//     }

//     let center = (image.width() as f32 / 2f32, image.height() as f32 / 2f32);

//     if (rotate_degree != 0.0) {
//         show_loading("旋转图片");
//         println!(
//             "draw_image>>旋转.. degree={} theta={}",
//             rotate_degree,
//             rotate_degree as f32 / 180.0
//         );
//         image = rotate_with_default(
//             &image,
//             center,
//             rotate_degree as f32 / 180.0,
//             Rgba([0u8, 0u8, 0u8, 0u8]),
//             Interpolation::Nearest,
//         );
//     }

//     imgtool::draw_image(x, y, &mut src_image, &image);
//     println!("draw_image>>结束.");
//     let (w, h) = (src_image.width(), src_image.height());
//     let mut raw_data = src_image.into_raw();
//     let len = raw_data.len();
//     let ptr = raw_data.as_mut_ptr() as *mut c_char;
//     std::mem::forget(raw_data);
//     set_app_field_json(
//         "draw_image_result",
//         &format!(
//             r#"{{
//         "imageDataLen": {},
//         "imageDataPtr": {},
//         "width": {},
//         "height": {}
//     }}"#,
//             len, ptr as i64, w, h
//         ),
//     );
// }

#[no_mangle]
pub fn resize_image(
    raw_image_data_buffer: *mut u8,
    buf_len: c_int,
    width: c_int,
    height: c_int,
    new_width: c_int,
    new_height: c_int,
) {
    let raw_image_data: Vec<u8> =
        unsafe { Vec::from_raw_parts(raw_image_data_buffer, buf_len as usize, buf_len as usize) };
    let raw_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, raw_image_data).unwrap();

    let new_image = image::imageops::resize(
        &raw_image,
        new_width as u32,
        new_height as u32,
        FilterType::Nearest,
    );

    let (w, h) = (new_image.width(), new_image.height());
    let mut raw_data = new_image.into_raw();
    let len = raw_data.len();
    let ptr = raw_data.as_mut_ptr() as *mut c_char;
    std::mem::forget(raw_data);
    set_app_field_json(
        "resize_image_result",
        &format!(
            r#"{{
        "imageDataLen": {},
        "imageDataPtr": {},
        "width": {},
        "height": {}
    }}"#,
            len, ptr as i64, w, h
        ),
    );
}

#[no_mangle]
pub fn rotate_image(
    raw_image_data_buffer: *mut u8,
    buf_len: c_int,
    width: c_int,
    height: c_int,
    rotate_degree: c_float
) {
    let raw_image_data: Vec<u8> =
        unsafe { Vec::from_raw_parts(raw_image_data_buffer, buf_len as usize, buf_len as usize) };
    let raw_image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, raw_image_data).unwrap();

    let center = (raw_image.width() as f32 / 2f32, raw_image.height() as f32 / 2f32);
    let new_image = rotate_with_default(
        &raw_image,
        center,
        rotate_degree as f32 / 180.0,
        Rgba([0u8, 0u8, 0u8, 0u8]),
        Interpolation::Nearest,
    );

    let (w, h) = (new_image.width(), new_image.height());
    let mut raw_data = new_image.into_raw();
    let len = raw_data.len();
    let ptr = raw_data.as_mut_ptr() as *mut c_char;
    std::mem::forget(raw_data);
    set_app_field_json(
        "rotate_image_result",
        &format!(
            r#"{{
        "imageDataLen": {},
        "imageDataPtr": {},
        "width": {},
        "height": {}
    }}"#,
            len, ptr as i64, w, h
        ),
    );
}

// #[no_mangle]
// pub fn sub_image(
//     src_image_data_buffer:*mut u8,
//     buf_len:c_int,
//     src_width: c_int,
//     src_height: c_int,

//     x:c_int,
//     y:c_int,
//     sub_width:c_int,
//     sub_height:c_int
// ){
//     let raw_image_data:Vec<u8> = unsafe{ Vec::from_raw_parts(src_image_data_buffer, buf_len as usize, buf_len as usize) };
//     let mut raw_image:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
//         src_width as u32,
//         src_height as u32,
//         raw_image_data).unwrap();

//     let sub_image = raw_image
//             .sub_image(x as u32, y as u32, sub_width as u32, sub_height as u32)
//             .to_image();
//     let (w,h) = (sub_image.width(), sub_image.height());
//     let mut raw_data = sub_image.into_raw();
//     // println!("sub_image raw_data.len={} {}x{}", raw_data.len(), w, h);

//     let len = raw_data.len();
//     let ptr = raw_data.as_mut_ptr() as *mut c_char;
//     std::mem::forget(raw_data);
//     set_app_field_json("sub_image_result",
//         &format!(r#"{{
//         "imageDataLen": {},
//         "imageDataPtr": {},
//         "width": {},
//         "height": {}
//     }}"#, len, ptr as i64, w, h));
// }

//检测人脸位置
#[no_mangle]
pub fn detect_face(
    image_data_buffer: *mut u8,
    buf_len: c_int,
    width: c_int,
    height: c_int,
    min_face_size: c_int,
    max_image_size: c_int,
) {
    // println!("detect_face:{:?},{:?},{:?},{:?}", image_data_buffer, buf_len, width, height);
    let image_data: Vec<u8> =
        unsafe { Vec::from_raw_parts(image_data_buffer, buf_len as usize, buf_len as usize) };
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width as u32, height as u32, image_data).unwrap();

    CONTEXT.with(|context| {
        let mut context = context.borrow_mut();
        let detector = context.detector.as_mut().unwrap();
        println!(
            "人脸识别: minsize={} 原图:{}x{} 最大边长:{}",
            min_face_size,
            image.width(),
            image.height(),
            max_image_size
        );
        let mut faces = face_off::detect_faces(&mut image, detector, max_image_size as u32);

        if faces.len() == 0 {
            delete_app_field("detect_face_result");
        } else {
            let rect = faces.remove(0);
            // println!("Rust.detect_face>>>人脸检测结果:{:?}", rect);

            set_app_field_json(
                "detect_face_result",
                &format!(
                    r#"{{
                    "x": {},
                    "y": {},
                    "width": {},
                    "height": {}
            }}"#,
                    rect.x, rect.y, rect.width, rect.height
                ),
            );
        }
    });
}

#[no_mangle]
pub fn init_detector() {
    //初始化人脸识别器
    let data = get_cascade_file_data();
    match face_off::init_detector(data) {
        Ok(d) => {
            CONTEXT.with(|context| {
                context.borrow_mut().detector = Some(d);
            });
        }
        Err(err) => {
            show_modal(&format!("初始化失败：{:?}", err));
        }
    }
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
fn main() {
    unsafe { js_set_module() };
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}
