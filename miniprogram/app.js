//app.js
App({
  onLaunch: function () {
  },
  getFileName: function(path){
    let idx = path.lastIndexOf("/");
    let pidx = path.lastIndexOf(".");
    var name = path.substring(idx + 1, pidx);
    return name.replace("_", ' ');
  },
  //压缩图片
  compressImage: function(path, canvasContext, callback){
    //console.log("-------------使用canvas压缩图片尺寸---------");
    let MAX = 720.0;
    wx.getImageInfo({
      src: path,
      success(res) {
        var width = res.width;
        var height = res.height;
        var new_width = width;
        var new_height = height;
        if (width > MAX || height > MAX){
          var new_width = width;
          var new_height = height;
          if (width > MAX){
            new_width = MAX;
            new_height = (height / width) * MAX;
          }
          if (new_height > MAX){
            new_height = MAX;
            new_width = (width / height) * MAX;
          }
        }
        new_width = parseInt(new_width);
        new_height = parseInt(new_height);
        //绘制
        // console.log("-------------绘制压缩图片:", new_width, new_height);
        canvasContext.drawImage(path, 0, 0, new_width, new_height);
        canvasContext.draw(false, function(res){
          // console.log("-------------压缩图片绘制成功:", res);
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
            }
          })
        });
      }
    });
  },

  globalData: {
    userDataPath: `${wx.env.USER_DATA_PATH}/`
  }
})