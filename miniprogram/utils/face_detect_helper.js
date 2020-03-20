const FACES = {

};


module.exports = {
  detectFace: function(imageData, width, height, minsize){
    if (!minsize){
      minsize = 80;
    }

    //检测人脸位置
    // console.log("开始检测人脸");
    var t = Date.now();
    //最大宽度400，加快人脸识别速度
    getApp().Module["_detect_face"](this.allocTypedArrayBuffer(imageData), imageData.length, width, height, minsize, 560);
    console.log("人脸检测耗时", Date.now() - t, "ms");

    return getApp().detect_face_result;
  },

  allocTypedArrayBuffer: function(array) {
    var buf = getApp().Module._malloc(array.length * array.BYTES_PER_ELEMENT);
    getApp().Module.HEAPU8.set(array, buf);
    // console.log('allocBuffer 长度:' + array.length+"指针:"+buf);
    return buf;
    //这里不调用free，由rust代码中from_raw自动释放
    //getApp().Module._free(buf);
  },

  // //缩放图片
  // resizeImage: function (canvasContext, imageInfo, newWidth, newHeight, cb){
  //   canvasContext
  // },

  //压缩图片
  compressImage: function (maxWidth, maxHeight, path, canvasContext, callback, userChoose) {
    // let MAX_WIDTH = 1200.0;
    // let MAX_HEIGHT = 2000.0;
    console.log("压缩图片:", path);
    let MAX_WIDTH = maxWidth;
    let MAX_HEIGHT = maxHeight;
    wx.getImageInfo({
      src: path,
      success(res) {
        var width = res.width;
        var height = res.height;
        var new_width = width;
        var new_height = height;
        if (width > MAX_WIDTH || height > MAX_HEIGHT) {
          var new_width = width;
          var new_height = height;
          if (width > MAX_WIDTH) {
            new_width = MAX_WIDTH;
            new_height = (height / width) * MAX_WIDTH;
          }
          if (new_height > MAX_HEIGHT) {
            new_height = MAX_HEIGHT;
            new_width = (width / height) * MAX_HEIGHT;
          }
        }
        new_width = parseInt(new_width);
        new_height = parseInt(new_height);
        //console.log("绘制大小", new_width, new_height);
        //绘制
        // console.log("-------------绘制压缩图片:", new_width, new_height);
        canvasContext.drawImage(path, 0, 0, new_width, new_height);
        canvasContext.draw(false, function (res) {
          if (userChoose){
            //保存临时文件，使用imgSecCheck检查图片是否违规
            wx.showLoading({
              title: "开始验证图片",
              mask: false,
            });
            wx.canvasToTempFilePath({
              x: 0,
              y: 0,
              width: new_width,
              height: new_height,
              destWidth: new_width,
              destHeight: new_height,
              fileType: "jpg",
              quality: 0.5,
              canvasId: 'scale-canvas',
              success(res) {
                console.log("imgSecCheck临时文件保存成功", res.tempFilePath);
                wx.showLoading({
                  title: "正在验证图片",
                  mask: false,
                });
                var tempFilePath = res.tempFilePath;
                wx.getFileSystemManager().readFile({
                  filePath: tempFilePath,
                  success: function (res) {
                    wx.showLoading({
                      title: "正在验证图片",
                      mask: false,
                    });
                    wx.cloud.callFunction({
                      name: 'imgSecCheck',
                      data: {
                        imgType: tempFilePath.split('.').pop(),
                        file: res.data
                      },
                      success(res) {
                        console.log('图片审查结果', res)
                        if (res.result.errCode == 0 || res.result.errCode == '0') {
                          console.log('图片经过校验,没有违法违规');
                          //绘制成功，获取图片数据
                          wx.canvasGetImageData({
                            canvasId: 'scale-canvas',
                            x: 0,
                            y: 0,
                            width: new_width,
                            height: new_height,
                            success(res) {
                              // console.log("-------------获取图片数据结果:", res);
                              callback(res);
                            },
                            fail(res) {
                              // console.log("-------------获取图片数据结果:", res);
                            }
                          });
                        } else {
                          callback();
                          console.log('图片不合规, 提示用户');
                          if (res.result.errCode == '87014') {
                            wx.hideLoading();
                            wx.showModal({
                              content: '存在敏感内容，请更换图片',
                              showCancel: false,
                              confirmText: '我知道了'
                            });
                          } else {
                            wx.hideLoading();
                            wx.showModal({
                              content: '图片不合规，请更换图片',
                              showCancel: false,
                              confirmText: '我知道了'
                            })
                          }
                        }
                      }, fail(res) {
                        callback();
                        console.log("图片验证失败", res);
                        wx.hideLoading();
                        wx.showModal({
                          content: '图片不合规，请更换图片',
                          showCancel: false,
                          confirmText: '我知道了'
                        });
                      }
                    });
                  },
                  fail: function (res) {
                    wx.hideLoading();
                    console.log(res);
                    wx.showModal({
                      content: '图片读取失败，请重试',
                      showCancel: false,
                      confirmText: '确定'
                    });
                  }
                });
              },
              fail(res) {
                console.log("imgSecCheck临时文件保存失败", res)
                wx.hideLoading();
                console.log(res);
                wx.showModal({
                  content: '临时文件保存失败，请重试',
                  showCancel: false,
                  confirmText: '确定'
                });
              }
            });
          }else{
            console.log("compressImage 自带图片");
            //绘制成功，获取图片数据
            wx.canvasGetImageData({
              canvasId: 'scale-canvas',
              x: 0,
              y: 0,
              width: new_width,
              height: new_height,
              success(res) {
                // console.log("-------------获取图片数据结果:", res);
                callback(res);
              },
              fail(res) {
                // console.log("-------------获取图片数据结果:", res);
              }
            });
          }
        });
      },
      fail(res) {
        console.log("getImageInfo", res);
        wx.hideLoading();
        console.log(res);
        wx.showModal({
          content: '图片读取失败，请重试',
          showCancel: false,
          confirmText: '确定'
        });
      }
    });
  },

  //截取压缩图片
  getSubImage: function (maxWidth, maxHeight, path, x_ratio, y_ratio, width_ratio, height_ratio, canvasContext, callback) {
    let MAX_WIDTH = maxWidth;
    let MAX_HEIGHT = maxHeight;
    wx.getImageInfo({
      src: path,
      success(res) {
        var width = res.width;
        var height = res.height;
        var new_width = width;
        var new_height = height;
        if (width > MAX_WIDTH || height > MAX_HEIGHT) {
          var new_width = width;
          var new_height = height;
          if (width > MAX_WIDTH) {
            new_width = MAX_WIDTH;
            new_height = (height / width) * MAX_WIDTH;
          }
          if (new_height > MAX_HEIGHT) {
            new_height = MAX_HEIGHT;
            new_width = (width / height) * MAX_HEIGHT;
          }
        }
        new_width = parseInt(new_width);
        new_height = parseInt(new_height);
        //console.log("绘制大小", new_width, new_height);
        //绘制
        // console.log("-------------绘制压缩图片:", new_width, new_height);
        canvasContext.drawImage(path, 0, 0, new_width, new_height);

        var x = parseInt(new_width * x_ratio);
        var y = parseInt(new_height * y_ratio);
        var width = parseInt(new_width * width_ratio);
        var height = parseInt(new_height * height_ratio);

        canvasContext.draw(false, function (res) {
          // console.log("-------------压缩图片绘制成功:", res);
          //绘制成功，获取图片数据
          wx.canvasGetImageData({
            canvasId: 'scale-canvas',
            x: x,
            y: y,
            width: width,
            height: height,
            success(res) {
              console.log("SubImage截取成功:", path);
              callback(res);
            },
            fail(res) {
              // console.log("-------------获取图片数据结果:", res);
            }
          })
        });
      },
      fail(res) {
        console.log("getImageInfo", res);
      }
    });
  },

  drawImage: function (maxWidth, maxHeight, path, canvasContext, callback) {
    let MAX_WIDTH = maxWidth;
    let MAX_HEIGHT = maxHeight;
    wx.getImageInfo({
      src: path,
      success(res) {
        var width = res.width;
        var height = res.height;
        var new_width = width;
        var new_height = height;
        if (width > MAX_WIDTH || height > MAX_HEIGHT) {
          var new_width = width;
          var new_height = height;
          if (width > MAX_WIDTH) {
            new_width = MAX_WIDTH;
            new_height = (height / width) * MAX_WIDTH;
          }
          if (new_height > MAX_HEIGHT) {
            new_height = MAX_HEIGHT;
            new_width = (width / height) * MAX_HEIGHT;
          }
        }
        new_width = parseInt(new_width);
        new_height = parseInt(new_height);
        //绘制
        canvasContext.drawImage(path, 0, 0, new_width, new_height);
        callback({ width: new_width, height: new_height });
      },
      fail(res) {
        console.log("getImageInfo", res);
      }
    });
  },
};