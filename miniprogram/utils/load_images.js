var md5 = require("md5");

let DEFAULT_IMAGES = [{ "_id": "5d15b8d853d5c1c508a3cf64", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/01_big.jpg", "color_type": "0", "face_height": 74, "face_width": 74, "face_x": 102, "face_y": 77, "id": "001", "name": "蒙娜丽莎", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/01.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf67", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mh2.jpg", "color_type": "0", "face_height": 46, "face_width": 46, "face_x": 116, "face_y": 100, "id": "002", "name": "画家与女儿像", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mh2_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf6a", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mh0.jpg", "color_type": "0", "face_height": 77, "face_width": 77, "face_x": 90, "face_y": 119, "id": "003", "name": "戴珍珠耳环的女孩", "rotate": 0, "scale": 1.1, "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mh0_small.jpg", "transx": 0.005, "transy": 0.01 }, { "_id": "5d15b8d853d5c1c508a3cf6c", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mh1.jpg", "color_type": "0", "face_height": 74, "face_width": 74, "face_x": 90, "face_y": 92, "id": "004", "name": "The Gypsy Girl", "rotate": 0, "scale": 1.05, "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mh1_small.jpg", "transx": 0.021, "transy": 0.01 }, { "_id": "5d15b8d853d5c1c508a3cf6e", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/02_big.jpg", "color_type": "0", "face_height": 77, "face_width": 77, "face_x": 119, "face_y": 94, "id": "005", "name": "莎玛丽夫人像", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/02.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf70", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/03.jpg", "color_type": "0", "face_height": 49, "face_width": 49, "face_x": 55, "face_y": 104, "id": "006", "name": "美国哥特式", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/03_s.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf72", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/Amedeo_Modigliani_042.jpg", "color_type": "0", "face_height": 120, "face_width": 120, "face_x": 66, "face_y": 122, "id": "007", "name": "Amedeo_Modigliani", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/Amedeo_Modigliani_sm.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf74", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/dt_mic.jpg", "color_type": "1", "face_height": 160, "face_width": 160, "face_x": 132, "face_y": 91, "id": "008", "name": "麦霸", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/03.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf76", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mogu0.jpg", "color_type": "1", "face_height": 65, "face_width": 65, "face_x": 70, "face_y": 55, "id": "009", "name": "蘑菇头", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mogu0_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf78", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mogu1.jpg", "color_type": "1", "face_height": 95, "face_width": 95, "face_x": 108, "face_y": 55, "id": "010", "name": "蘑菇头", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mogu1_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf7c", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mogu2.jpg", "color_type": "1", "face_height": 90, "face_width": 90, "face_x": 80, "face_y": 84, "id": "011", "name": "蘑菇头", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mogu2_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf7f", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/mogu4.jpg", "color_type": "1", "face_height": 77, "face_width": 77, "face_x": 188, "face_y": 71, "id": "012", "name": "蘑菇头", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/mogu4_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf82", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/panda0.jpg", "color_type": "1", "face_height": 89, "face_width": 89, "face_x": 54, "face_y": 9, "id": "013", "name": "熊猫表情", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/panda0_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf86", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/panda1.jpg", "color_type": "1", "face_height": 120, "face_width": 120, "face_x": 123, "face_y": 40, "id": "014", "name": "熊猫表情", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/panda1_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf8b", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/panda2.jpg", "color_type": "1", "face_height": 103, "face_width": 103, "face_x": 143, "face_y": 49, "id": "015", "name": "熊猫表情", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/panda2_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf8f", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/panda3.jpg", "color_type": "1", "face_height": 84, "face_width": 84, "face_x": 90, "face_y": 69, "id": "016", "name": "熊猫表情", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/panda3_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf92", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/panda4.jpg", "color_type": "1", "face_height": 110, "face_width": 110, "face_x": 127, "face_y": 52, "id": "017", "name": "熊猫表情", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/panda4_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf95", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/19650.jpg", "color_type": "1", "face_height": 96, "face_width": 96, "face_x": 161, "face_y": 56, "id": "018", "name": "斗图", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/19650_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf99", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/19651.jpg", "color_type": "1", "face_height": 68, "face_width": 68, "face_x": 18, "face_y": 12, "id": "019", "name": "斗图", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/19651_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf9c", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/19652.jpg", "color_type": "1", "face_height": 64, "face_width": 64, "face_x": 114, "face_y": 29, "id": "020", "name": "斗图", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/19652_small.jpg" }, { "_id": "5d15b8d853d5c1c508a3cf9f", "big": "cloud://pub-5f9af0.7075-pub-5f9af0/big/19653.jpg", "color_type": "1", "face_height": 61, "face_width": 61, "face_x": 98, "face_y": 40, "id": "021", "name": "斗图", "small": "cloud://pub-5f9af0.7075-pub-5f9af0/small/19653_small.jpg" }];


var pageIndex = 0;
var pageSize = 20;
var allImageList = [];

function onPageLoad(data) {

  var end = (!data) || data.length < pageSize;
  if (data) {
    allImageList = allImageList.concat(data);
  }
  if (!end) {
    pageIndex += 1;
    loadPage(pageIndex, pageSize);
    return;
  }
  //console.log(JSON.stringify(allImageList));
  if (allImageList && allImageList.length>0){
    //存储到本地
    wx.setStorage({
      key: 'images',
      data: allImageList,
    });
    console.log("最新图片列表加载完成!");
  }else{
    console.log("最新图片列表加载失败!");
  }
}

function loadPage(pageIndex, pageSize) {
  // console.log("读取数据库图片列表>loadPage("+pageIndex+","+pageSize+")");
  var page = this;
  const db = wx.cloud.database();
  db.collection('preset_images')
    .skip(pageIndex * pageSize)
    .limit(pageSize)
    .orderBy('id', 'asc')
    .get({
      success(res) {
        onPageLoad(res.data);
      },
      fail(res) {
        onPageLoad();
      }
    });
}

module.exports = {
  pullImages: function(){
    //加载最新的图片列表
    //查询所有图片列表(每次最多20条)
    console.log("查询所有图片列表(每次最多20条)");
    pageIndex = 0;
    loadPage(pageIndex, pageSize);
  },

  loadImages: function(cb){
    wx.getStorage({
      key: 'images',
      success: function(res){
        cb(res.data);
      },
      fail: function (res) {
        wx.setStorage({
          key: 'images',
          data: DEFAULT_IMAGES,
        });
        cb(DEFAULT_IMAGES);
      }
    });
  },

  //下载大图
  getBigImagePath: function (url, callback) {
    if (!url.startsWith("cloud://")){
      //本地图片直接返回
      // console.log("getBigImagePath"+url+" 本地文件直接返回path");
      callback(url);
      return;
    }
    let fsm = wx.getFileSystemManager();
    let filePath = `${wx.env.USER_DATA_PATH}/` + "bigimage_" + md5(url) + ".jpg";
    // console.log("getBigImagePath, url=", url, filePath);
    //检查本地文件是否存在
    fsm.getFileInfo({
      filePath: filePath,
      success(res) {
        console.log("大图本地文件已存在");
        callback(filePath);
      },
      fail(res) {
        console.log("大图本地文件不存在，从网络下载");
        //下载文件
        wx.cloud.downloadFile({
          fileID: url,
          success: res => {
            // get temp file path
            var tempFilePath = res.tempFilePath;
            //如果本地大图已经超过10张，删除最早的一张
            fsm.readdir({
              dirPath: `${wx.env.USER_DATA_PATH}/`,
              success(res) {
                //console.log("本地文件列表数", res.files.length);
                var bigImageFileInfoList = [];
                for (var i = 0; i < res.files.length; i++) {
                  if (res.files[i].startsWith("bigimage_")) {
                    bigImageFileInfoList.push(res.files[i]);
                  }
                }
                //console.log("本地缓存大图文件列表数:", bigImageFileInfoList.length);
                if (bigImageFileInfoList.length > 9) {
                  //删除最早的
                  while (bigImageFileInfoList.length > 9) {
                    //var info = bigImageFileInfoList.splice(0, 1);
                    var info = bigImageFileInfoList.pop();
                    fsm.unlink({
                      filePath: `${wx.env.USER_DATA_PATH}/` + info,
                      success(res) {
                        console.log("删除成功", res);
                      },
                      fail(res) {
                        console.log("删除失败", res);
                      }
                    });
                  }
                }

                fsm.saveFile({
                  filePath: filePath,
                  tempFilePath: tempFilePath,
                  success(res) {
                    //返回保存的路径
                    callback(filePath);
                  },
                  fail(res) {
                    //console.log('saveFile', res);
                    callback();
                  }
                });
              },
              fail(res) {
                //console.log('getSavedFileList', res);
                callback();
              }
            });
          },
          fail: err => {
            console.log('downloadFile', err);
            callback();
          }
        });
      },
    });
  },

  //下载小图
  getSmallImagePath: function (url, callback) {
    let fsm = wx.getFileSystemManager();
    let filePath = `${wx.env.USER_DATA_PATH}/` + "smimage_" + md5(url) + ".jpg";
    //console.log("getSmallImagePath, url=", url, filePath);
    //检查本地文件是否存在
    fsm.getFileInfo({
      filePath: filePath,
      success(res) {
        console.log("小图本地文件已存在");
        callback(filePath);
      },
      fail(res) {
        console.log("小图本地文件不存在，从网络下载");
        //下载文件
        wx.cloud.downloadFile({
          fileID: url,
          success: res => {
            // get temp file path
            var tempFilePath = res.tempFilePath;
            //如果本地大图已经超过10张，删除最早的一张
            fsm.readdir({
              dirPath: `${wx.env.USER_DATA_PATH}/`,
              success(res) {
                //console.log("本地文件列表数", res.files.length);
                var smImageFileInfoList = [];
                for (var i = 0; i < res.files.length; i++) {
                  if (res.files[i].startsWith("smimage_")) {
                    smImageFileInfoList.push(res.files[i]);
                  }
                }
                //console.log("本地缓存大图文件列表数:", smImageFileInfoList.length);
                if (smImageFileInfoList.length > 19) {
                  //删除最早的
                  while (smImageFileInfoList.length > 19) {
                    //var info = smImageFileInfoList.splice(0, 1);
                    var info = smImageFileInfoList.pop();
                    fsm.unlink({
                      filePath: `${wx.env.USER_DATA_PATH}/` + info,
                      success(res) {
                        console.log("删除成功", res);
                      },
                      fail(res) {
                        console.log("删除失败", res);
                      }
                    });
                  }
                }

                fsm.saveFile({
                  filePath: filePath,
                  tempFilePath: tempFilePath,
                  success(res) {
                    //返回保存的路径
                    callback(filePath);
                  },
                  fail(res) {
                    console.log('saveFile', res);
                    callback();
                  }
                });
              },
              fail(res) {
                console.log('getSavedFileList', res);
                callback();
              }
            });
          },
          fail: err => {
            console.log('downloadFile', err);
            callback();
          }
        });
      },
    });
  },
};