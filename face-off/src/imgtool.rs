use image::{ImageBuffer, Rgb, Rgba};
// use std::collections::HashMap;

// pub fn _copy_hue1(
//     src: &ImageBuffer<Rgba<u8>, Vec<u8>>,
//     target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
// ) {
//     //统计src灰度级
//     let mut src_gray_map: Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
//     let (width, _height) = (src.width(), src.height());
//     for (y, col) in src.chunks(width as usize * 4).enumerate() {
//         for (x, pixel) in col.chunks(4).enumerate() {
//             let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
//                 as i32;
//             if pixel[3] > 0 {
//                 let mut contains = false;
//                 let mut c_id = 0;
//                 for (i, (egray, _)) in src_gray_map.iter().enumerate() {
//                     if gray == *egray {
//                         contains = true;
//                         c_id = i;
//                         break;
//                     }
//                 }

//                 if contains {
//                     (src_gray_map[c_id].1).1.push((x, y));
//                 } else {
//                     src_gray_map.push((gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)])));
//                 }
//             }
//         }
//     }

//     //统计target灰度级
//     let (width, _height) = (target.width(), target.height());
//     let mut target_gray_map: Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
//     for (y, col) in target.chunks(width as usize * 4).enumerate() {
//         for (x, pixel) in col.chunks(4).enumerate() {
//             let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
//                 as i32;
//             if pixel[3] > 0 {
//                 let mut contains = false;
//                 let mut c_id = 0;
//                 for (i, (egray, _)) in target_gray_map.iter().enumerate() {
//                     if gray == *egray {
//                         contains = true;
//                         c_id = i;
//                         break;
//                     }
//                 }

//                 if contains {
//                     (target_gray_map[c_id].1).1.push((x, y));
//                 } else {
//                     target_gray_map
//                         .push((gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)])));
//                 }
//             }
//         }
//     }

//     //循环每个user头像的灰度级，找到与之最相近的famous图像灰度级以及对应的颜色
//     for (gray, (_color, points)) in &target_gray_map {
//         let mut closest_color = None;
//         let mut min_distance = 255;
//         for (fgray, fvalue) in &src_gray_map {
//             let dist = (fgray - gray).abs();
//             if dist < min_distance {
//                 closest_color = Some(fvalue.0);
//                 min_distance = dist;
//             }
//         }
//         //更新颜色
//         // println!("closest_gray={}", closest_gray);
//         //value.0 = famous_gray_map.get(&closest_gray).unwrap().0;
//         // let color = famous_gray_map.get(&closest_gray).unwrap().0;
//         let color = closest_color.unwrap();
//         for (x, y) in points {
//             let pixel = &mut target[(*x as u32, *y as u32)];
//             pixel[0] = color[0];
//             pixel[1] = color[1];
//             pixel[2] = color[2];
//         }
//     }
// }

pub fn copy_hue(src: &ImageBuffer<Rgba<u8>, Vec<u8>>, target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    //统计src灰度级
    let mut src_gray_map: Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
    let (width, _height) = (src.width(), src.height());
    for (y, col) in src.chunks(width as usize * 4).enumerate() {
        for (x, pixel) in col.chunks(4).enumerate() {
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                as i32;
            if pixel[3] > 0 {
                let mut contains = false;
                let mut c_id = 0;
                for (i, (egray, _)) in src_gray_map.iter().enumerate() {
                    if gray == *egray {
                        contains = true;
                        c_id = i;
                        break;
                    }
                }

                if contains {
                    (src_gray_map[c_id].1).1.push((x, y));
                } else {
                    src_gray_map.push((gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)])));
                }
            }
        }
    }

    //将底图的灰度级排序
    src_gray_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //统计target灰度级
    let (width, _height) = (target.width(), target.height());
    let mut target_gray_map: Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
    for (y, col) in target.chunks(width as usize * 4).enumerate() {
        for (x, pixel) in col.chunks(4).enumerate() {
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                as i32;
            if pixel[3] > 0 {
                let mut contains = false;
                let mut c_id = 0;
                for (i, (egray, _)) in target_gray_map.iter().enumerate() {
                    if gray == *egray {
                        contains = true;
                        c_id = i;
                        break;
                    }
                }

                if contains {
                    (target_gray_map[c_id].1).1.push((x, y));
                } else {
                    target_gray_map
                        .push((gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)])));
                }
            }
        }
    }

    target_gray_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    //将图2对应序号的颜色全部改为图1对应序号下的颜色
    if src_gray_map.len() > 0 && target_gray_map.len() > 0 {
        for i in 0..target_gray_map.len() {
            let srci = if src_gray_map.len() > i {
                i
            } else {
                src_gray_map.len() - 1
            };
            //替换每一个颜色
            let (_color, points) = &target_gray_map[i].1;
            let (color, _) = &src_gray_map[srci].1;
            for (x, y) in points {
                let pixel = &mut target[(*x as u32, *y as u32)];
                pixel[0] = color[0];
                pixel[1] = color[1];
                pixel[2] = color[2];
            }
        }
    }
}

// pub fn _copy_hue_hashmap(
//     src: &ImageBuffer<Rgba<u8>, Vec<u8>>,
//     target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
// ) {
//     //统计famous灰度级
//     let (width, _height) = (src.width(), src.height());
//     let mut famous_gray_map: HashMap<i32, (Rgb<u8>, Vec<(usize, usize)>)> = HashMap::new();
//     for (y, col) in src.chunks(width as usize * 4).enumerate() {
//         for (x, pixel) in col.chunks(4).enumerate() {
//             let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
//                 as i32;
//             if pixel[3] > 0 {
//                 if famous_gray_map.contains_key(&gray) {
//                     famous_gray_map.get_mut(&gray).unwrap().1.push((x, y));
//                 } else {
//                     famous_gray_map
//                         .insert(gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)]));
//                 }
//             }
//         }
//     }
//     // println!("灰度数量:{}", famous_gray_map.len()); //16色，16个灰度级

//     //统计user灰度级
//     let (width, _height) = (target.width(), target.height());
//     let mut user_gray_map: HashMap<i32, (Rgb<u8>, Vec<(usize, usize)>)> = HashMap::new();
//     for (y, col) in target.chunks(width as usize * 4).enumerate() {
//         for (x, pixel) in col.chunks(4).enumerate() {
//             let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
//                 as i32;
//             if pixel[3] > 0 {
//                 if user_gray_map.contains_key(&gray) {
//                     user_gray_map.get_mut(&gray).unwrap().1.push((x, y));
//                 } else {
//                     user_gray_map.insert(gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)]));
//                 }
//             }
//         }
//     }
//     // println!("灰度数量:{}", user_gray_map.len()); //16色，16个灰度级

//     //循环每个user头像的灰度级，找到与之最相近的famous图像灰度级以及对应的颜色
//     for (gray, value) in &user_gray_map {
//         let mut closest_gray = 0;
//         let mut min_distance = 255;
//         for (fgray, _fvalue) in &famous_gray_map {
//             let dist = (fgray - gray).abs();
//             if dist < min_distance {
//                 closest_gray = *fgray;
//                 min_distance = dist;
//             }
//         }
//         //更新颜色
//         let (_, points) = value;
//         println!("closest_gray={}", closest_gray);
//         //value.0 = famous_gray_map.get(&closest_gray).unwrap().0;
//         let color = famous_gray_map.get(&closest_gray).unwrap().0;
//         for (x, y) in points {
//             let pixel = &mut target[(*x as u32, *y as u32)];
//             pixel[0] = color[0];
//             pixel[1] = color[1];
//             pixel[2] = color[2];
//         }
//     }
// }

/*


//裁剪椭圆人脸区域(椭圆裁剪只是把椭圆以外的区域设置为透明，不改变图片大小)，裁边处理
//fight_compute: 是否进行斗图处理
fn clip_ellipse(image: ImageBuffer<Rgba<u8>, Vec<u8>>, fight_compute: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    //椭圆测试
    let (iw, ih) = (image.width(), image.height());
    let (cx, cy) = (iw as f64 / 2.0, ih as f64 * 0.45);
    let (a, b) = (cx * 0.8, cy*1.4);
    let mut user_head_data: Vec<u8> = image.into_raw();
    //首先剪切椭圆区域
    for (y, row) in user_head_data.chunks_mut(iw as usize * 4).enumerate() {
        for (x, col) in row.chunks_mut(4).enumerate() {
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;
            if (dx * dx) / (a * a) + (dy * dy) / (b * b) > 1.0 {
                //不在椭圆内的点全透明
                col[0] = 255;
                col[1] = 0;
                col[2] = 0;
                col[3] = 0;
            }
        }
    }
    //剪切额头部分(去掉刘海)
    let top = (ih as f64*0.15) as usize;
    for i in (0..top*iw as usize*4).step_by(4){
        user_head_data[i] = 0;
        user_head_data[i+1] = 0;
        user_head_data[i+2] = 0;
        user_head_data[i+3] = 0;
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
let border = (iw as f64*0.12) as i32;
for row in user_head_data.chunks_mut(iw as usize * 4){
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
//top是额头得起始y, bottom是额头半透明得底部, dt是额头半透明高度
let dt = (ih as f64*0.1) as usize;
let bottom = top+dt;
for (y, row) in user_head_data.chunks_mut(iw as usize * 4).enumerate(){
for col in row.chunks_mut(4) {
if y<=bottom && col[3]>0{
let per = 1.0-(bottom-y) as f64/dt as f64;
let op = (per*255.0) as u8;
if op<col[3]{
col[3] = op;
}
}
}
if y>bottom{
break;
}
}

//下边半透明
let stride = iw as usize * 4;//行步进
let mut top = (ih as f64*0.9) as usize;
let border = (ih as usize-top) as f64;
let start = stride*top;
for py in (start..user_head_data.len()).step_by(stride){
for px in (0..stride).step_by(4){
let p = py+px;
let a = &mut user_head_data[p+3];
if *a>0{
let per = (ih as i32 - top as i32) as f64/border;
let op = (per*255.0) as u8;
if op<*a{
 *a = op;
}
}
}
top += 1;
}

//高于一定亮度的颜色变白
if fight_compute{
for pixel in user_head_data.chunks_mut(4){
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

ImageBuffer::from_raw(iw, ih, user_head_data).unwrap()
}

*/

///绘制图片
pub fn draw_image(
    x: i32,
    y: i32,
    target: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    let (target_width, target_height) = (target.width() as i32, target.height() as i32);

    let (x, y, width, height) = (x, y, image.width() as i32, image.height() as i32);

    /* 颜色混合
        一般数学计算中，颜色取值是：R,G,B∈[0,255],A∈[0,1]
        所以对于一般的颜色混合有：Color(RGBA)=Color(R1,G1,B1,A1)+Color(R2,G2,B2,A2)

        标准的颜色混合算法如下：
        A=1−(1−α1)∗(1−α2)

        R=(α1*R1+(1−α1)*α2*R2)/A
        G=(α1*G1+(1−α1)*α2*G2)/A
        B=(α1*B1+(1−α1)*α2*B2)/A
    */
    let mut sy = 0;
    for cy in y..y + height {
        let mut sx = 0;
        for cx in x..x + width {
            if cx >= 0 && cx < target_width && cy >= 0 && cy < target_height {
                let image_pixel = image[(sx, sy)];
                let (r1, g1, b1, a1) = (
                    image_pixel[0] as f64,
                    image_pixel[1] as f64,
                    image_pixel[2] as f64,
                    image_pixel[3] as f64 / 255.0,
                );
                let pixel = &mut target[(cx as u32, cy as u32)];
                let (r2, g2, b2, a2) = (
                    pixel[0] as f64,
                    pixel[1] as f64,
                    pixel[2] as f64,
                    pixel[3] as f64 / 255.0,
                );

                let a = 1.0 - (1.0 - a1) * (1.0 - a2);
                let r = (a1 * r1 + (1.0 - a1) * a2 * r2) / a;
                let g = (a1 * g1 + (1.0 - a1) * a2 * g2) / a;
                let b = (a1 * b1 + (1.0 - a1) * a2 * b2) / a;

                pixel[0] = r as u8;
                pixel[1] = g as u8;
                pixel[2] = b as u8;
                pixel[3] = (a * 255.0) as u8;
            }
            sx += 1;
        }

        sy += 1;
    }
}
