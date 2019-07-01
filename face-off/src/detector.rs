#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use image::GrayImage;

extern "C" {
    pub fn find_objects(
        rcsq: *mut f32,
        maxndetections: ::std::os::raw::c_int,
        cascade: *mut ::std::os::raw::c_void,
        angle: f32,
        pixels: *mut ::std::os::raw::c_void,
        nrows: ::std::os::raw::c_int,
        ncols: ::std::os::raw::c_int,
        ldim: ::std::os::raw::c_int,
        scalefactor: f32,
        stridefactor: f32,
        minsize: f32,
        maxsize: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cluster_detections(rcsq: *mut f32, n: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn update_memory(
        slot: *mut ::std::os::raw::c_int,
        memory: *mut f32,
        counts: *mut ::std::os::raw::c_int,
        nmemslots: ::std::os::raw::c_int,
        maxslotsize: ::std::os::raw::c_int,
        rcsq: *mut f32,
        ndets: ::std::os::raw::c_int,
        maxndets: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

const MAXNDETECTIONS: i32 = 2048;
const NMEMSLOTS: i32 = 5;
const MAXSLOTSIZE: i32 = 1024;
use std::ffi::c_void;

/// Pcio算法
pub struct Detector {
    cascade: Vec<u8>,  //要检测的对象数据
    minsize: i32,      //设置查找对象的*最小*大小（默认值为64）
    maxsize: i32,      //设置查找对象的*最大*大小（默认值为1024）
    scalefactor: f32, //在多尺度检测过程中重新缩放窗口的数量（默认值为1.2）;增加该值可以减少检测次数，提高处理速度;例如，如果您在移动设备上使用pico，则设置为1.2
    stridefactor: f32, //在相邻检测之间移动窗口的数量（默认值为0.1，即10％）：增加此值会导致检测次数减少和处理速度更快;例如，如果您想要非常高的召回率，则设置为0.05。
    angle: f32,
    rcsq: [f32; 4 * MAXNDETECTIONS as usize],
    memory: [f32; 4 * NMEMSLOTS as usize * MAXSLOTSIZE as usize],
    counts: [i32; NMEMSLOTS as usize],
    slot: [i32; 1],
    qthreshold: f32, //检测质量阈值（> = 0.0）：将丢弃估计质量低于此阈值的所有检测（默认值为5.0）
    noclustering: bool, //true for test
    noupdatememory: bool, //默认关闭
}

#[derive(Debug)]
pub struct Area {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub score: f32,
}

impl Detector {
    pub fn new(cascade: Vec<u8>) -> Detector {
        Detector {
            cascade,
            minsize: 64,
            maxsize: 1024,
            scalefactor: 1.2,
            stridefactor: 0.1,
            angle: 0.0,
            rcsq: [0.0f32; 4 * MAXNDETECTIONS as usize],
            memory: [0.0f32; 4 * NMEMSLOTS as usize * MAXSLOTSIZE as usize],
            counts: [0i32; NMEMSLOTS as usize],
            slot: [1],
            qthreshold: 5.0,
            noclustering: false,
            noupdatememory: true,
        }
    }

    pub fn set_minsize(&mut self, minsize: i32) {
        self.minsize = minsize;
    }

    pub fn set_maxsize(&mut self, maxsize: i32) {
        self.maxsize = maxsize;
    }

    pub fn set_scalefactor(&mut self, scalefactor: f32) {
        self.scalefactor = scalefactor;
    }

    pub fn set_stridefactor(&mut self, stridefactor: f32) {
        self.stridefactor = stridefactor;
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn set_qthreshold(&mut self, qthreshold: f32) {
        self.qthreshold = qthreshold;
    }

    pub fn set_noupdatememory(&mut self, noupdatememory: i32) {
        self.noupdatememory = if noupdatememory == 0 { false } else { true };
    }

    /// 检测人脸
    pub fn detect_face(&mut self, image: &GrayImage) -> Vec<Area> {
        let (width, height) = (image.width(), image.height());
        self.find_objects(image, height as i32, width as i32, width as i32)
    }

    /// find objects
    ///
    /// pixels: image data
    ///
    /// nrows: eq height
    ///
    /// ncols: eq width
    ///
    /// ldim: widthStep
    ///
    pub fn find_objects(&mut self, pixels: &[u8], nrows: i32, ncols: i32, ldim: i32) -> Vec<Area> {
        let maxsize = if self.maxsize < nrows && self.maxsize < ncols {
            self.maxsize as f32
        } else {
            (nrows as f32).min(ncols as f32)
        };
        let mut ndetections = unsafe {
            find_objects(
                self.rcsq.as_mut_ptr(),
                MAXNDETECTIONS,
                self.cascade.as_mut_ptr() as *mut c_void,
                self.angle,
                pixels.as_ptr() as *mut c_void,
                nrows,
                ncols,
                ldim,
                self.scalefactor,
                self.stridefactor,
                self.minsize as f32,
                maxsize,
            )
        };

        if !self.noupdatememory {
            ndetections = unsafe {
                update_memory(
                    self.slot.as_mut_ptr(),
                    self.memory.as_mut_ptr(),
                    self.counts.as_mut_ptr(),
                    NMEMSLOTS,
                    MAXSLOTSIZE,
                    self.rcsq.as_mut_ptr(),
                    ndetections,
                    MAXNDETECTIONS,
                )
            };
        }

        if !self.noclustering {
            ndetections = unsafe { cluster_detections(self.rcsq.as_mut_ptr(), ndetections) };
        }
        let mut areas = vec![];
        for i in 0..ndetections as usize {
            let score = self.rcsq[4 * i + 3];
            if score >= self.qthreshold {
                areas.push(Area {
                    x: self.rcsq[4 * i + 1],
                    y: self.rcsq[4 * i + 0],
                    radius: self.rcsq[4 * i + 2] / 2.0,
                    score: score,
                });
            }
        }
        areas
    }
}

// use image::{ConvertBuffer, GrayImage, ImageBuffer, Rgba};
// use std::ffi::c_void;
// use std::time::Instant;

// const MAXNDETECTIONS: i32 = 2048;
// const NMEMSLOTS: i32 = 5;
// const MAXSLOTSIZE: i32 = 1024;

// pub fn test() {
//     println!("start.");
//     let image = image::open("img.jpg").unwrap().to_rgba();
//     let (width, height) = (image.width(), image.height());
//     let gray: GrayImage = image.convert();

//     let cascade = include_bytes!("../pico/rnt/cascades/facefinder").to_vec();

//     let mut pico = Pico::new(cascade);
//     pico.set_minsize(20);

//     let n = 1;
//     let t = Instant::now();

//     //4.627ms  scalefactor=1.2, min=55,max=1000,

//     let mut areas = vec![];

//     // for _ in 0..1000 {
//     areas = pico.find_objects(&gray, height as i32, width as i32, width as i32);
//     // }

//     println!(
//         "areas = {:?} 个数:{} {}ms",
//         areas,
//         areas.len(),
//         t.elapsed().as_millis() as f64 / n as f64
//     );

//     let mut image = image::open("img.jpg").unwrap();
//     for area in areas{
//         imageproc::drawing::draw_hollow_circle_mut(&mut image, (area.x as i32, area.y as  i32), area.radius as i32, Rgba([255, 0, 0, 255]));
//     }
//     image.save("result.jpg").unwrap();
// }
