use image::{ImageBuffer, Rgb,Rgba};

pub fn copy_hue(src:&ImageBuffer<Rgba<u8>, Vec<u8>>, target:&mut ImageBuffer<Rgba<u8>, Vec<u8>>){
    //统计src灰度级
    let mut src_gray_map:Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
    let (width, _height) = (src.width(), src.height());
    for (y, col) in src.chunks(width as usize * 4).enumerate() {
        for (x, pixel) in col.chunks(4).enumerate() {
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                as i32;
            if pixel[3] > 0 {
                let mut contains = false;
                let mut c_id = 0;
                for (i, (egray, _)) in src_gray_map.iter().enumerate(){
                    if gray == *egray{
                        contains = true;
                        c_id = i;
                        break;
                    }
                }

                if contains{
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
    let mut target_gray_map:Vec<(i32, (Rgb<u8>, Vec<(usize, usize)>))> = vec![];
    for (y, col) in target.chunks(width as usize * 4).enumerate() {
        for (x, pixel) in col.chunks(4).enumerate() {
            let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32)
                as i32;
            if pixel[3] > 0 {
                let mut contains = false;
                let mut c_id = 0;
                for (i, (egray, _)) in target_gray_map.iter().enumerate(){
                    if gray == *egray{
                        contains = true;
                        c_id = i;
                        break;
                    }
                }

                if contains{
                    (target_gray_map[c_id].1).1.push((x, y));
                } else {
                    target_gray_map.push((gray, (Rgb([pixel[0], pixel[1], pixel[2]]), vec![(x, y)])));
                }
            }
        }
    }

    target_gray_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());


    //将图2对应序号的颜色全部改为图1对应序号下的颜色
    if src_gray_map.len()>0 && target_gray_map.len()>0{
        for i in 0..target_gray_map.len(){
            let srci = if src_gray_map.len()>i{i}else{src_gray_map.len()-1};
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