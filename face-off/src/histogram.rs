use std::cmp::min;
use std::ptr::null_mut;

///使用直方图进行颜色量化
///使用示例：
///原文地址：https://pixelero.wordpress.com/2014/11/12/just-saying-hi-or-color-quantization-using-histogram/
///
pub struct ColorQuantizationHistogram {
    //所需的色阶数量
    levels: usize,
    //最大聚类数量
    iterations: usize,
}

impl ColorQuantizationHistogram {
    pub fn new(levels: usize) -> ColorQuantizationHistogram {
        ColorQuantizationHistogram {
            levels,
            iterations: 100,
        }
    }

    pub fn convert_image(&self, data: &[u8], _w: u32, _h: u32) -> Vec<u8> {
        //计算直方图
        let hst = Self::calculate_histograms(data);

        //将k-means应用于每个通道
        let arr_red = Self::kmeans1(&hst[0], min(8, self.levels));
        let arr_green = Self::kmeans1(&hst[1], min(8, self.levels));
        let arr_blue = Self::kmeans1(&hst[2], min(8, self.levels));

        //只用一个通道计算一个新图像，
        //由k-means聚类索引形成的值：
        // red_index + #numberOfReds * green_index +#numberOfReds * #numberOfGreens * blue_index
        let mut rgb: Vec<u16> = Vec::with_capacity(data.len() >> 2);
        let f1 = arr_red.len();
        let f2 = f1 * arr_green.len();

        let rarr = Self::histogram_to_lookup_by_index(&arr_red, 1);
        let garr = Self::histogram_to_lookup_by_index(&arr_green, f1 as u16);
        let barr = Self::histogram_to_lookup_by_index(&arr_blue, f2 as u16);

        //将图像映射到这些3d集群索引：
        for pixel in data.chunks(4) {
            //rgb[a]
            rgb.push(rarr[pixel[0] as usize] + garr[pixel[1] as usize] + barr[pixel[2] as usize]);
        }

        //形成一个新的三维指数直方图
        let mut rgb_hist: Vec<usize> = vec![0; f2 * arr_blue.len()];
        for i in 0..rgb.len() {
            rgb_hist[rgb[i] as usize] += 1;
        }

        //我需要两个数组来处理顺序，尽管可能有空的直方图箱
        let mut rgb_3d: Vec<*mut RGBItem> = vec![];
        let mut rgb_all: Vec<Box<RGBItem>> = vec![];
        let tres = 0;
        let mut i = 0usize;
        for ib in 0..arr_blue.len() {
            for ig in 0..arr_green.len() {
                for ir in 0..arr_red.len() {
                    let item = Box::new(RGBItem::new(
                        arr_red[ir].x as f64,
                        arr_green[ig].x as f64,
                        arr_blue[ib].x as f64,
                        rgb_hist[i],
                    ));

                    let ptr = Box::into_raw(item);
                    rgb_all.push(unsafe { Box::from_raw(ptr) });
                    if rgb_hist[i] > tres {
                        rgb_3d.push(ptr);
                    }

                    i += 1;
                }
            }
        }

        rgb_3d.sort_by(|a, b| {
            let a = unsafe { a.as_ref() }.unwrap();
            let b = unsafe { b.as_ref() }.unwrap();
            b.count.cmp(&a.count)
        });

        //选择具有最大条目数的直方图箱，初始质心
        let mut centroids = vec![];
        for i in 0..min(rgb_3d.len(), self.levels) {
            let item = unsafe { rgb_3d[i].as_ref() }.unwrap();
            centroids.push(RGBItem::new(item.r, item.g, item.b, item.count));
        }

        let mut indx = vec![0; rgb_3d.len()];
        let mut iter = 0;
        let mut do_loop = true;

        while do_loop && iter < self.iterations {
            iter += 1;
            for i in 0..centroids.len() {
                centroids[i].count = 1;
            }
            do_loop = false;
            //  找到每个数据条目最近的质心：
            for i in 0..rgb_3d.len() {
                let mut mi = 0;
                let item = unsafe { rgb_3d[i].as_ref() }.unwrap();
                let mut dist = centroids[mi].distance(&item);
                for j in 1..centroids.len() {
                    let d2 = centroids[j].distance(&item);
                    if d2 < dist {
                        dist = d2;
                        mi = j;
                    }
                }
                if mi != indx[i] {
                    //  一些值改变了，我们继续运行迭代：
                    do_loop = true;
                    indx[i] = mi;
                }
            }
            //  将质心更新到新位置：
            for i in 0..rgb_3d.len() {
                centroids[indx[i]].combine(unsafe { rgb_3d[i].as_mut() }.unwrap());
            }
        }

        //为了更快的性能，我们为每个通道计算一个大小为256的查找表：
        let mut rarr = vec![];
        let mut garr = vec![];
        let mut barr = vec![];

        for i in 0..rgb_all.len() {
            let k = &rgb_all[i];
            rarr.push(k.red());
            garr.push(k.green());
            barr.push(k.blue());
        }

        //将索引映射回rgb-space
        let mut data = vec![];
        for j in 0..rgb.len() {
            let k = rgb[j] as usize;
            data.push(rarr[k] as u8);
            data.push(garr[k] as u8);
            data.push(barr[k] as u8);
            data.push(255); //alpha
        }

        data
    }

    //迭代图像，计算每个rgb通道的直方图
    fn calculate_histograms(data: &[u8]) -> [[u32; 256]; 3] {
        let mut rarr: [u32; 256] = [0; 256];
        let mut garr: [u32; 256] = [0; 256];
        let mut barr: [u32; 256] = [0; 256];

        //在非常大的图像的情况下，这可以仅通过每第2，第3或第N像素的样本来加速：
        // i + = 6，i + = 10，i + = 2 + 4 * N
        for pixel in data.chunks(4) {
            //rgba
            rarr[pixel[0] as usize] += 1;
            garr[pixel[1] as usize] += 1;
            barr[pixel[2] as usize] += 1;
        }
        [rarr, garr, barr]
    }

    //将像素值0 ... 255转换为簇索引0 ...＃levels-1
    fn histogram_to_lookup_by_index(arr: &[ArrayItem], f: u16) -> [u16; 256] {
        let mut arr2 = [0; 256];
        for i in 0..arr.len() {
            for j in arr[i].min.trunc() as usize..=arr[i].max as usize {
                arr2[j as usize] = f * i as u16;
            }
        }
        arr2
    }

    //应用一维k均值
    // arr：值为0 ... 255的数组
    // N：所需的簇数：
    fn kmeans1(arr: &[u32; 256], n: usize) -> Vec<ArrayItem> {
        let mut min = 0;
        let mut max = arr.len() - 1;
        //通过从白端删除空条目来规范化范围
        while arr[max] == 0 && min < max {
            max -= 1;
        }
        //找到一个空通道，返回一个0 ... 255的单个簇
        if min == max {
            let mut c = ArrayItem::new();
            c.max = arr.len() as f64 - 1.0;
            c.x = c.max / 2.0;
            return vec![c];
        }

        //从黑色端删除空条目
        while arr[min] == 0 {
            min += 1;
        }

        //通过划分数据范围的初始质心，例如N = 2：完整直方图的两个簇：0 ... 127和128 ... 255
        let mut c = vec![];
        let d = (max as f64 - min as f64) / (2.0 * n as f64);
        let mut m = min as f64;

        for _ in 0..n {
            let mut item = ArrayItem::new();
            item.min = m.trunc();
            c.push(item);
            m += 2.0 * d;
        }

        for i in 0..n - 1 {
            c[i].max = c[i + 1].min - 1.0;
        }

        c[0].min = 0.0;
        c[n - 1].max = arr.len() as f64 - 1.0;

        //将arr-values添加到相应的集群
        let mut ic = 0;
        for i in 0..arr.len() {
            if i as f64 > c[ic].max {
                ic += 1;
            }
            c[ic].add(i as f64, arr[i] as f64);
        }

        //更新质心在一个维度上很简单：只需检查边界处的值是否属于正确的群集，如果需要，请更改
        let mut do_loop = true;
        let mut iter = 0;
        while do_loop && iter < 1000 {
            iter += 1;
            do_loop = false;
            for i in 0..c.len() - 1 {
                let mut j = c[i].max;
                // 将较低的值移动到较高的群集
                if c[i].distance(j) > c[i + 1].distance(j) {
                    c[i].subtract(j, arr[j as usize] as f64);
                    c[i + 1].add(j, arr[j as usize] as f64);
                    c[i].max -= 1.0;
                    c[i + 1].min -= 1.0;
                    do_loop = true;
                } else {
                    //将较高的值移动到较低的集群：
                    j += 1.0; //j<255.0后加的
                    if j < 255.0 && c[i].distance(j) < c[i + 1].distance(j) {
                        c[i].add(j, arr[j as usize] as f64);
                        c[i + 1].subtract(j, arr[j as usize] as f64);
                        c[i].max += 1.0;
                        c[i + 1].min += 1.0;
                        do_loop = true;
                    }
                }
            }
        }

        //x =质心坐标为整数
        for i in 0..c.len() {
            c[i].x = c[i].centroid().trunc();
        }

        c
    }
}

//一维k均值算法所需的Struct
#[derive(Debug)]
struct ArrayItem {
    min: f64, //0~1
    max: f64, //0~255
    x: f64,   //0~255
    sum_m: f64,
    sum_mx: f64,
}

impl ArrayItem {
    fn new() -> ArrayItem {
        ArrayItem {
            min: 0.0,
            max: 0.0,
            x: 0.0,
            sum_m: 1.0,
            sum_mx: 0.0,
        }
    }

    //'value'是直方图中的条目数
    fn add(&mut self, x: f64, value: f64) {
        self.sum_mx += x * value;
        self.sum_m += value;
    }

    fn subtract(&mut self, x: f64, value: f64) {
        self.sum_mx -= x * value;
        self.sum_m -= value;
    }

    fn centroid(&self) -> f64 {
        self.sum_mx / self.sum_m
    }

    //一维距离abs（x-x2）
    fn distance(&self, x: f64) -> f64 {
        let d = x - self.centroid();
        if d < 0.0 {
            -d
        } else {
            d
        }
    }
}

//三维k均值算法所需的Struct
struct RGBItem {
    ///RGB组件
    r: f64,
    g: f64,
    b: f64,

    //输入数量
    count: usize,

    //当组合两个集群时，链接到其他RGBItem
    mapped_to: *mut RGBItem,
}

impl RGBItem {
    // fn to_string(&self) -> String {
    //     //format!("r:{},g:{},b:{},count:{}", self.r, self.g, self.b, self.count)
    //     format!(
    //         "{} {} {} {} {:?} mapped_to.is_null()?{}",
    //         self.r,
    //         self.g,
    //         self.b,
    //         self.count,
    //         self.mapped_to,
    //         self.mapped_to.is_null()
    //     )
    // }
    fn new(r: f64, g: f64, b: f64, count: usize) -> RGBItem {
        RGBItem {
            r,
            g,
            b,
            count,
            mapped_to: null_mut(),
        }
    }

    //RGB空间中两个颜色之间的平方距离
    fn distance(&self, item: &RGBItem) -> f64 {
        let dr = self.r - item.r;
        let dg = self.g - item.g;
        let db = self.b - item.b;
        dr * dr + dg * dg + db * db
    }

    //添加另一个RGBItem的条目，更新新的RGB值： 参数中的item指向Self
    fn combine(&mut self, item: &mut RGBItem) {
        let c = self.count + item.count;
        let f = 1.0 / c as f64;
        self.r = f * (self.count as f64 * self.r + item.count as f64 * item.r);
        self.g = f * (self.count as f64 * self.g + item.count as f64 * item.g);
        self.b = f * (self.count as f64 * self.b + item.count as f64 * item.b);
        self.count = c;
        //形成从'item'到'self'的链接
        item.mapped_to = self;
    }

    fn red(&self) -> f64 {
        if self.mapped_to.is_null() {
            self.r
        } else {
            unsafe { self.mapped_to.as_mut() }.unwrap().red()
        }
    }

    fn green(&self) -> f64 {
        if self.mapped_to.is_null() {
            self.g
        } else {
            unsafe { self.mapped_to.as_mut() }.unwrap().green()
        }
    }

    fn blue(&self) -> f64 {
        if self.mapped_to.is_null() {
            self.b
        } else {
            unsafe { self.mapped_to.as_mut() }.unwrap().blue()
        }
    }
}
