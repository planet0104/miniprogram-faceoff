//index.js
//获取应用实例
const app = getApp();

const TYPE_BASE = 0;
const TYPE_FRONT = 1;

const DEFAULT_HEAD_PATH = '/static/head.jpg';

require("face-off.js");

var chooseType = TYPE_BASE;
var famousImagePath = null;
var usersImagePath = null;
var currentBaseImagePathBack = '/static/preset/Mona_Lisa.jpg';
//压缩处理以后的图像
var baseImageData = null;
var frontImageData = null;

var imageBoxRect = null;//图片自动缩放以后的位置
var canvasContext = null;//渲染文字用的画布
var labelInfo = {x:0};//文字大小和位置

var imageScale = 1.0;
var imageRotate = 0;

var colorMatch = false;

Page({
  data: {
    showShare: false,
    showChooseDialog: false,
    showPresetImages: true,
    selectDialogContentTop: '130rpx',

    showSelectFrontTip: false,
    currentBaseImagePath: '/static/preset/Mona_Lisa.jpg',
    currentBaseImageName: 'Mona Lisa',
    currentFrontImageName: '点击选择',
    currentFrontImagePath: DEFAULT_HEAD_PATH,
    convertImageData: '/static/preset/Mona_Lisa.jpg',

    dtRotateValue: 40,
    dtScaleValue: 30,
  },

  /**
   * 用户点击右上角分享
   */
  onShareAppMessage: function () {
    this.hideShare();
    return {
      title: '趣图换脸',
      path: 'pages/index/index',
    }
  },

  showShare: function () {
    this.setData({ showShare: true });
  },
  hideShare: function () {
    this.setData({ showShare: false });
  },

  scaleHead: function(event){
    console.log("scaleHead...");
    if (colorMatch || this.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
      this.setData({ dtScaleValue: 30, showSelectFrontTip: true });
      return;
    }

    //缩放 正负 30%
    let value = event.detail.value - 30;
    imageScale = 1.0 + value/100.0;

    var page = this;
    wx.showLoading({
      title: '缩放头像',
      mask: true,
    });
    //创建
    let image = getApp().face_off.resize(imageScale, imageRotate);
    if (image == null) {
      wx.showModal({
        title: "温馨提示",
        content: "小程序初始化失败，请稍先删除小程序，然后重试。",
        showCancel: false
      });
    } else {
      let data = new Uint8Array(image);
      const base64 = wx.arrayBufferToBase64(data.buffer);
      page.setData({ convertImageData: "data:image/gif;base64," + base64 });
    }
    wx.hideLoading();
  },

  rotateHead: function(event){
    console.log("rotateHead...");
    if (colorMatch || this.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
      this.setData({ dtRotateValue:40, showSelectFrontTip: true });
      return;
    }

    //正负40度调整
    let value = event.detail.value-40;
    imageRotate = value;

    var page = this;
    wx.showLoading({
      title: '旋转头像',
      mask: true,
    });
    //创建
    let image = getApp().face_off.resize(imageScale, imageRotate);
    if (image == null) {
      wx.showModal({
        title: "温馨提示",
        content: "小程序初始化失败，请稍先删除小程序，然后重试。",
        showCancel: false
      });
    } else {
      let data = new Uint8Array(image);
      const base64 = wx.arrayBufferToBase64(data.buffer);
      page.setData({ convertImageData: "data:image/gif;base64," + base64 });
    }
    wx.hideLoading();
  },

  onMovableChange: function (event){
    console.log("拖动:", event.detail);
    labelInfo.x = event.detail.x;
    labelInfo.y = event.detail.y;
  },

  setImageBoxRect:function(textWidth){
    var page = this;
    wx.createSelectorQuery().select('#img-show').boundingClientRect(function (rect) {
      console.log("setImageBoxRect:", rect);
      imageBoxRect = rect;
      if (textWidth){
        labelInfo.x = rect.width/2-textWidth/2;
      }
      page.setData({ moveArea: { color: '#fff', show: true, labelLeft: labelInfo.x, labelTop:rect.bottom-45, left: rect.left, top: rect.top, width: rect.width, height: rect.height }});
    }).exec();
  },

  canvasIdErrorCallback(e) {
    console.error(e.detail.errMsg)
  },

  onReady: function(){
  },
  
  //换脸
  createImage: function(){
    colorMatch = false;
    if (baseImageData==null){
      wx.hideLoading();
      wx.showModal({
        title: '温馨提示',
        content: '请重新选择底图',
        showCancel: false,
      });
      return;
    }
    if (frontImageData == null) {
      wx.hideLoading();
      wx.showModal({
        title: '温馨提示',
        content: '请重新选择前景图',
        showCancel: false,
      });
      return;
    }
    var page = this;
    wx.showLoading({
      title: '请耐心等待',
      mask: true,
    });
    //重置缩放旋转参数
    imageScale = 1.0;
    imageRotate = 0;
    this.setData({ dtScaleValue: 30});
    this.setData({ dtRotateValue: 40 });
    //创建
    let image = getApp().face_off.create(
      baseImageData.buffer, baseImageData.width, baseImageData.height,
      frontImageData.buffer, frontImageData.width, frontImageData.height, false);
    if (image == null){
        wx.showModal({
          title: "温馨提示",
          content: "小程序初始化失败，请稍先删除小程序，然后重试。",
          showCancel: false
        });
    } else if (typeof image == "string"){
      wx.showModal({
        title: "没有找到人脸",
        content: image,
        showCancel: true,
        cancelText: "确定",
        confirmText: "颜色匹配",
        success: function(res){
          if (res.confirm) {
            colorMatch = true;
            wx.showLoading({
              title: '正在转换',
              mask: true
            });
            //风格转换
            let image = getApp().face_off.convert_style(
              baseImageData.buffer, baseImageData.width, baseImageData.height,
              frontImageData.buffer, frontImageData.width, frontImageData.height);
              wx.hideLoading();
              if (image == null) {
                wx.showModal({
                  title: "温馨提示",
                  content: "转换失败，请重试",
                  showCancel: false
                });
              }else{
                let data = new Uint8Array(image);
                const base64 = wx.arrayBufferToBase64(data.buffer);
                page.setData({ convertImageData: "data:image/gif;base64," + base64 });
              }
          } else if (res.cancel) {}
        }
      });
    }else{
      let data = new Uint8Array(image);
      const base64 = wx.arrayBufferToBase64(data.buffer);
      // console.log("创建的buffer", data.buffer);
      page.setData({ convertImageData: "data:image/gif;base64," + base64 });
    }
    wx.hideLoading();
  },

  //显示底图
  selectPresetImage: function (event) {
    var path = event.currentTarget.dataset.path;
    this.closeChooseDialog();
    this.updateBaseImage(path);
  },

  //更新底图
  updateBaseImage: function(path){
    var page = this;
    currentBaseImagePathBack = path;
    this.setData({ currentBaseImageName: app.getFileName(path), convertImageData: path, currentBaseImagePath: path }, function(){
      page.setImageBoxRect();
    });
    wx.showLoading({
      title: '读取图片',
      mask: true
    });
    this.loadBaseImage(currentBaseImagePathBack, function(){
      if (baseImageData != null && page.data.currentFrontImagePath != DEFAULT_HEAD_PATH) {
        page.createImage();
      } else {
        wx.hideLoading();
      }
    });
  },

  //事件处理函数
  closeChooseDialog: function() {
    this.setData({ showChooseDialog: false});
  },

  //切换不同模式：匹配/原图/斗图
  chooseMode: function(){
    if (this.data.currentFrontImagePath == DEFAULT_HEAD_PATH){
      this.setData({ showSelectFrontTip: true });
      return;
    }
  },
  //保存图片
  saveImage: function(){
    var page = this;
    if (this.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
      this.setData({ showSelectFrontTip: true });
      return;
    }

    var save = function(){
      wx.showLoading({
        title: '保存图片',
        mask: true
      });
      let fsm = wx.getFileSystemManager();
      let filePath = `${wx.env.USER_DATA_PATH}/` + 'painting.jpg';
      let data = wx.base64ToArrayBuffer(page.data.convertImageData.replace("data:image/gif;base64,", ""));
      //console.log(data);
      fsm.writeFile({
        filePath: filePath, data: data,
        success: function (res) {
          wx.saveImageToPhotosAlbum({
            filePath: filePath,
            fail: function(res){},
            success(res) {
              console.log("相册保存成功", res);
              //图片已保存到相册
              wx.showModal({
                title: '保存成功',
                content: '图片已保存到相册，请到相册查看，或进入“预览”界面长按分享',
                showCancel: true,
                cancelText: '关闭',
                confirmText: '预览',
                success(res) {
                  if (res.confirm) {
                    wx.previewImage({
                      urls: [filePath]
                    });
                  } else if (res.cancel) {
                    
                  }
                }
              });
            }
          })
        },
        fail: function (res) {
          console.log("图片保存失败", res);
          wx.showModal({
            title: "温馨提示",
            content: '图片保存失败！',
            showCancel: false
          });
        },
        complete: function () {
          wx.hideLoading();
        }
      });
    };

    //获取相册授权
    wx.getSetting({
      success(res) {
        if(!res.authSetting['scope.writePhotosAlbum']){
          wx.openSetting({
          });
        }else{
          save();
        }
      }
    });
  },
  openChooseDialog: function(event){
    var type = event.currentTarget.dataset.type;
    chooseType = type == 'base' ? TYPE_BASE : TYPE_FRONT;
    if(type!='base'){
      this.setData({ showSelectFrontTip: false });
    }
    this.setData({
      showChooseDialog: true,
      showPresetImages: type=='base'?true:false,
      selectDialogContentTop: type=='base'?'100rpx':'430rpx'
    });
  },

  //从相册选择照片
  chooseImage: function (event) {
    this.closeChooseDialog();
    var page = this;
    var sourceType = event.currentTarget.dataset.type;
    wx.chooseImage({
      // sizeType: ['compressed'],
      sizeType: ['original'],
      sourceType: sourceType == 'camera' ? ['camera'] : ['album'],
      success: res => {
        if (res.tempFilePaths && res.tempFilePaths.length > 0) {
          if (chooseType == TYPE_BASE){
            if (res.tempFilePaths[0] != currentBaseImagePathBack){
              page.updateBaseImage(res.tempFilePaths[0]);
            }
          }else{
            var choosePath = res.tempFilePaths[0];
            page.setData({ currentFrontImageName: app.getFileName(choosePath), currentFrontImagePath: choosePath });
            wx.showLoading({
              title: '读取图片',
              mask: true
            });
            page.loadFrontImage(choosePath, function(){
              if (frontImageData != null) page.createImage();
            });
          }
        }
      }
    });
  },

  onShow: function(){
  },

  onFaceOffInit: function(res){
    var page = this;
    if(res){
      wx.showModal({
        title: "温馨提示",
        content: res,
        showCancel: false
      });
      wx.hideLoading();
      return;
    }

    this.loadBaseImage(currentBaseImagePathBack, function(){
      wx.hideLoading();
    });
    wx.getStorage({
      key: 'showtip',
      fail: function (res) {
        wx.setStorage({
          key: 'showtip',
          data: 'showtip',
        });
        page.setData({ showSelectFrontTip: true });
      }
    });
    wx.hideLoading();
    console.log("初始化成功:" + (Date.now() - getApp().t) + "ms");
  },

  onLoad: function () {
    //获取相册授权
    wx.getSetting({
      success(res) {
        if (!res.authSetting['scope.writePhotosAlbum']) {
          wx.authorize({
            scope: 'scope.writePhotosAlbum',
            success() {}
          })
        }
      }
    });

    canvasContext = wx.createCanvasContext("scale-canvas");

    getApp().page = this;
    var page = this;
    wx.showLoading({
      title: '正在启动',
      mask: true
    });
    getApp().t = Date.now();
    getApp().face_off.init();
  },

  loadImage: function(path, cb){
    app.compressImage(path, canvasContext, function(res){
      cb({
        buffer: res.data.buffer,
        width: res.width,
        height: res.height
      });
    });
  },

  //加载前景图
  loadFrontImage: function(path, cb){
    this.loadImage(path, function(data){
      frontImageData = data;
      cb();
    });
  },

  //加载底图
  loadBaseImage: function(path, cb){
    this.loadImage(path, function (data) {
      baseImageData = data;
      cb();
    });
  }
})
