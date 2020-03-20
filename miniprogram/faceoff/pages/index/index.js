//index.js
//获取应用实例
const app = getApp();
var init = false;
var smallBaseLoad = false;
var canvasLoad = false;

/**
 数据定义:
  imageData: Uint8ClampedArray,

  imageInfo: {
    imageData,
    width,
    height
  }

 */

const TYPE_BASE = 0;
const TYPE_FRONT = 1;
const WINDOW_WIDTH = wx.getSystemInfoSync().windowWidth;
function rpx_to_px(rpx) {
  return rpx / 750.0 * WINDOW_WIDTH;
}
function px_to_rpx(px) {
  return px * 750.0 * WINDOW_WIDTH;
}
//图片展示最大宽度和高度
const MAX_CANVAS_WIDTH = rpx_to_px(400.0);
const MAX_CANVAS_HEIGHT = rpx_to_px(610.0);
const CANVAS_PARENT_HEIGHT = rpx_to_px(630.0);
var currentCanvasWidth = 0;
var currentCanvasHeight = 0;

const NO_FACE_TIP = "如果图片中的人脸太小，可先在相册中剪切以后再识别";

const DEFAULT_HEAD_PATH = '/static/head.jpg';

var faceOff = require("face-off.js");
var faceDetectHelper = require("../../../utils/face_detect_helper.js");
var imageLoader = require("../../../utils/load_images.js");

var chooseType = TYPE_BASE;
var famousImagePath = null;
var usersImagePath = null;
//缩小处理以后的图像
var frontImage = {};

var imageBoxRect = null;//图片自动缩放以后的位置
var canvasContext = null;
var imageCanvasContext = null;

var userFaceRect; //用户头像位置
var megicHeadImageInfo;//魔法转换以后的用户头像图片

var imageScale = 1.0;
var imageRotate = 0;
var imageTransX = 0;
var imageTransY = 0;

//--------- 输入文字相关 ------------
var textColor = 'white';
var textColors = ['white', 'black', 'red', 'yellow', 'green', 'blue'];
var colorLevelsArray = [256, 128, 64, 56, 48, 40, 32, 16, 8, 4, 2];
var labelText = "";
var bindText = "";
var labelImage = null;
var labelX = 0;//文字实际在图片中的位置
var labelY = 0;//文字实际在图片中的位置
//------------------------------------

var colorMatch = false;
var selectBaseImage = {
  localPath: null,
  name: null,
  smallPath: null,
};

// 在页面中定义插屏广告
let interstitialAd = null
let nextShowTime = 0;
//标记用户是否使用过，使用过才显示广告
var savedPhoto = false;

Page({
  data: {
    colorLevels: ['256色', '128色', '64色', '56色', '48色', '40色', '32色', '16色', '8色', '4色', '2色'],
    showShare: false,
    showChooseDialog: false,
    showPresetImages: true,
    selectDialogContentTop: '130rpx',
    imageCanvasContextWidth: rpx_to_px(400.0),//第一张图:1200x1815
    imageCanvasContextHeight: rpx_to_px(605.0),

    //----------输入文字的提示----------------
    labelTipLeft: 0,
    labelTipTop: 0,
    showLabelTip: false,
    //----------输入文字相关 ----------------
    showLabel: false,
    labelColor: '#fff',
    labelText: '输入文字',
    labelTop: 0,
    labelTop: 0,
    isInputTextHidden: true,
    textColorId: 0,
    textColor: '白色',
    textColorArray: ['白色', '黑色', '红色', '黄色', '绿色', '蓝色'],
    labelTextSize: 36,
    //---------------------------------

    showSelectFrontTip: false,
    currentBaseImagePath: '/static/01.jpg',
    currentBaseImageName: '蒙娜丽莎',
    currentFrontImageName: '点击选择',
    currentFrontImagePath: DEFAULT_HEAD_PATH,
    convertImageData: '/static/01.jpg',
    imageBoxRect: {
      left: 50,
      top: 50,
      width: 50,
      height: 50
    },

    dtRotateValue: 60,
    dtScaleValue: 30,
    imageTransX: 0,
    imageTransX: 0,

    userFaceRectRotate: 0,
    userFaceRectScale: 1.0,
    showUserHeadImage: false,
    
    imageListCol0: [],
    imageListCol1: [],
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
    if (colorMatch || this.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
      this.setData({ dtScaleValue: 30, showSelectFrontTip: true });
      return;
    }

    //缩放 正负 30%
    let value = event.detail.value - 30;
    imageScale = 1.0 + value/100.0;

    this.applyUserHeadScale();
  },

  applyUserHeadScale: function(){
    var newWidth = userFaceRect.width * imageScale;
    var newHeight = userFaceRect.height * imageScale;
    var newX = userFaceRect.x - (newWidth - userFaceRect.width) / 2.0;
    var newY = userFaceRect.y - (newHeight - userFaceRect.height) / 2.0;

    this.setData({
      userFaceRect: {
        x: newX,
        y: newY,
        width: newWidth,
        height: newHeight
      }
    });
  },

  onImageTouch: function(event){
    if (event && event.detail){
      labelX = event.detail.x-event.currentTarget.offsetLeft;
      labelY = event.detail.y - event.currentTarget.offsetTop;
      this.setData({ showLabelTip: false, showLabel:true, labelLeft: event.detail.x, labelTop: event.detail.y});
      if (labelImage==null){
        this.setData({ isInputTextHidden: false});
      }
    }
  },

  moveLeft: function(){
    if (this.data.showUserHeadImage){
      imageTransX -= 1;
      this.setData({ imageTransX: imageTransX });
    }
  },

  moveRight: function () {
    if (this.data.showUserHeadImage) {
      imageTransX += 1;
      this.setData({ imageTransX: imageTransX });
    }
  },

  moveUp: function () {
    if (this.data.showUserHeadImage) {
      imageTransY -= 1;
      this.setData({ imageTransY: imageTransY });
    }
  },

  moveDown: function () {
    if (this.data.showUserHeadImage) {
      imageTransY += 1;
      this.setData({ imageTransY: imageTransY });
    }
  },

  rotateHead: function(event){
    if (colorMatch || this.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
      this.setData({ dtRotateValue:60, showSelectFrontTip: true });
      return;
    }

    //正负60度调整
    let value = event.detail.value-60;
    imageRotate = value;

    this.setData({ userFaceRectRotate: imageRotate});
  },

  canvasIdErrorCallback(e) {
    console.error(e.detail.errMsg)
  },

  onReady: function(){
    
  },
  setImageBoxRect: function () {
    var page = this;
    this.clearText();

    let imgWidth = selectBaseImage.imageInfo.width;
    let imgHeight = selectBaseImage.imageInfo.height;

    var scale = MAX_CANVAS_WIDTH / imgWidth;
    var canvasWidth = MAX_CANVAS_WIDTH;
    let canvasHeight = imgHeight*scale;
    if(canvasHeight>MAX_CANVAS_HEIGHT){
      canvasHeight = MAX_CANVAS_HEIGHT;
      scale = canvasHeight/imgHeight;
      canvasWidth = imgWidth*scale;
    }
    this.setData({ convertImageData: selectBaseImage.localPath, imageCanvasContextWidth: parseInt(canvasWidth), imageCanvasContextHeight: parseInt(canvasHeight)});

    imageBoxRect = {
      left: (WINDOW_WIDTH-canvasWidth)/2,
      top: (CANVAS_PARENT_HEIGHT-canvasHeight)/2+rpx_to_px(20),
      width: canvasWidth,
      height: canvasHeight,
    };

    page.setData({ imageBoxRect: imageBoxRect, labelTipLeft: imageBoxRect.left, labelTipTop: imageBoxRect.top + imageBoxRect.height-rpx_to_px(20), showLabelTip: true, });
    // imageCanvasContext.drawImage(selectBaseImage.localPath, 0, 0, canvasWidth, canvasHeight);
    // imageCanvasContext.draw();
    currentCanvasWidth = canvasWidth;
    currentCanvasHeight = canvasHeight;
  /*
    wx.createSelectorQuery().select('#img-show').boundingClientRect(function (rect) {
      //这里取到的是父元素的rect，还需要根据宽度计算图片实即的rect
      // console.log("rect:", rect);
      let imgWidth = selectBaseImage.imageInfo.width;borderBoxMarginTop
      let imgHeight = selectBaseImage.imageInfo.height;
      let sw = rect.width;
      let sh = (rect.width/imgWidth)*imgHeight;

      var realRect = {
        left: rect.left,
        right: rect.right,
        top: rect.top+ (rect.height-sh)/2.0,
        bottom: rect.top + (rect.height - sh) / 2.0+sh,
        width: rect.width,
        height: sh
      };
      // var borderBoxMarginTop = (rect.height - sh) / 2.0;
      // console.log("borderBoxMarginTop=" + borderBoxMarginTop);
      // page.setData({ borderBoxMarginTop: parseInt(borderBoxMarginTop)});
      
      // console.log("realRect:", realRect);
      imageBoxRect = realRect;
      //labelX = 50;
      //labelY = rect.height-50;
      // this.setData({ labelLeft: event.detail.x, labelTop: event.detail.y });
      page.setData({ labelTipLeft: realRect.left, labelTipTop: realRect.top + realRect.height, showLabelTip: true, });
    }).exec();
    */
  },

  replaceHeadError: function(msg){
    wx.hideLoading();
    wx.showModal({
      title: '温馨提示',
      content: msg,
      showCancel: false,
    });
  },
  
  //换脸
  replaceHead: function (colorLevels){
    colorMatch = false;
    if (!selectBaseImage.imageInfo){
      this.replaceHeadError('请重新选择底图');
      return;
    }
    if (!frontImage.imageInfo) {
      this.replaceHeadError('请重新选择前景图');
      return;
    }
    var page = this;
    
    //创建
    if (!selectBaseImage.faceInfo){
      this.replaceHeadError('底图中未检测到人脸，请重新选择！' + NO_FACE_TIP);
      return;
    }

    if(!frontImage.faceInfo){
      this.replaceHeadError('前景中未检测到人脸，请重新选择！' + NO_FACE_TIP);
      return;
    }

    if (!colorLevels) {
      colorLevels = parseInt(selectBaseImage.colorType) > 0 ? 40 : 64;
    }

    //按照展示图片比例变换底图人脸区域大小
    var scale = imageBoxRect.width / selectBaseImage.imageInfo.width;
    var x = selectBaseImage.faceInfo.faceRect.x;
    var y = selectBaseImage.faceInfo.faceRect.y;
    var width = selectBaseImage.faceInfo.faceRect.width;
    var height = selectBaseImage.faceInfo.faceRect.height;
    userFaceRect = {
      x: parseInt(imageBoxRect.left + x * scale),
      y: parseInt(imageBoxRect.top + y * scale),
      real_y: parseInt(imageBoxRect.top + y * scale),
      real_x: parseInt(imageBoxRect.left + x * scale),
      width: parseInt(width * scale),
      height: parseInt(height * scale)
    };

    wx.showLoading({
      title: '颜色变换...',
      mask: false,
    });

    //头像色调变换
    getApp().Module["_face_megic"](
      /*//前景图人脸
        front_head_image_buffer:*mut u8,
        front_head_image_buffer_len: c_int,
        front_head_image_width: c_int,
        front_head_image_height: c_int,
      */
      faceDetectHelper.allocTypedArrayBuffer(frontImage.faceInfo.imageInfo.imageData),
      frontImage.faceInfo.imageInfo.imageData.length,
      frontImage.faceInfo.imageInfo.width,
      frontImage.faceInfo.imageInfo.height,
      /*//底图人脸数据
          base_head_image_buffer:*mut u8,
          base_head_image_buffer_len: c_int,
          base_head_image_width: c_int,
          base_head_image_height: c_int,
      */
      faceDetectHelper.allocTypedArrayBuffer(selectBaseImage.faceInfo.imageInfo.imageData),
      selectBaseImage.faceInfo.imageInfo.imageData.length,
      selectBaseImage.faceInfo.imageInfo.width,
      selectBaseImage.faceInfo.imageInfo.height,
      colorLevels,//色阶数
    );
    // console.log("_face_megic耗时ms:", Date.now() - t); t = Date.now();

    var image = {
      imageData: this.getImageDataFromPtr(getApp().face_megic_result),
      width: getApp().face_megic_result.width,
      height: getApp().face_megic_result.height
    };
    megicHeadImageInfo = image;
    // console.log("getImageDataFromPtr耗时ms:", Date.now() - t); t = Date.now();

    wx.showLoading({
      title: '生成图片...',
      mask: true,
    });

    //头像图片转换成png
    getApp().Module["_create_png"](
      faceDetectHelper.allocTypedArrayBuffer(image.imageData),
      image.imageData.length,
      image.width,
      image.height
    );
    wx.hideLoading();
    if (getApp().create_png_result) {
      image = this.getImageDataFromPtr(getApp().create_png_result);
      let data = new Uint8Array(image);
      const base64 = "data:image/jpeg;base64," + wx.arrayBufferToBase64(data.buffer);
      this.setData({userHeadImageData: base64, showUserHeadImage: true });
      this.applyUserHeadScale();
    } else {
      wx.showModal({
        title: "温馨提示",
        content: "png生成失败，请退出重试。",
        showCancel: false
      });
    }
  },

  //显示底图
  selectPresetImage: function (event, cb) {
    this.closeChooseDialog();
    var idx = parseInt(event.currentTarget.dataset.id);
    var list = event.currentTarget.dataset.list;
    var selected = null;
    if(list==0){
      selected = this.data.imageListCol0[idx];
    }else{
      selected = this.data.imageListCol1[idx];
    }
    var page = this;
    wx.showLoading({
      title: '读取图片',
      mask: true
    });
    //下载小图
    imageLoader.getSmallImagePath(selected.small, function (path) {
      if (path) {
        selectBaseImage.localPath = path;
        selectBaseImage.smallPath = selected.small;
        selectBaseImage.bigPath = selected.big;
        selectBaseImage.name = selected.name;
        selectBaseImage.dbFaceRect = {
          x: selected.face_x,
          y: selected.face_y,
          width: selected.face_width,
          height: selected.face_height
        };
        selectBaseImage.colorType = selected.color_type;
        page.updateBaseImage();
        if (cb) cb();
      } else {
        wx.hideLoading();
        wx.showModal({
          title: "温馨提示",
          content: '图片下载失败！',
          showCancel: false
        });
      }
    });
  },

  //更新底图
  updateBaseImage: function(userChoose){
    var page = this;
    wx.showLoading({
      title: '读取图片',
      mask: true
    });
    page.loadBaseImage(selectBaseImage.localPath, function (isError) {
      if (!isError){
        page.setData({ currentBaseImageName: selectBaseImage.name, currentBaseImagePath: selectBaseImage.smallPath });
      }
      if (selectBaseImage.imageInfo && page.data.currentFrontImagePath != DEFAULT_HEAD_PATH) {
        page.setData({ imageTransX: 0.0, imageTransY: 0.0, dtRotateValue: 60, dtScaleValue: 30 });
        imageScale = 1.0;
        imageRotate = 0.0;
        imageTransX = 0.0;
        imageTransY = 0.0;
        page.replaceHead();
      } else {
        wx.hideLoading();
      }
    }, userChoose);
  },

  //事件处理函数
  closeChooseDialog: function() {
    this.setData({showChooseDialog: false});
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

    wx.showLoading({
      title: '读取大图',
      mask: true
    });
    /*
      优化速度 :
      1、首先将底图大图绘制到画布(最大1200x2000)
      2、调用rust代码，对头像进行缩放、旋转，然后将头像的imageData 放入画布
      3、将文字的imageData绘制到画布
      4、调用画布的save保存图片
    */
    //下载底图大图，分两种，预置图网络下载，本地图本地读取
    imageLoader.getBigImagePath(selectBaseImage.bigPath, function (path) {
      if (path) {
        var save = function () {
          //1、首先将底图大图绘制到画布

          wx.showLoading({
            title: '绘制图片',
            mask: true
          });
          faceDetectHelper.drawImage(1200.0, 2000.0, path, canvasContext, function (drawSize) {
            var image = null;

            var newFinalImageData = null;

            //首先按照用户调整的比例、角度，应用到小图的坐标上(image_view)
            var small_width_ratio = selectBaseImage.faceInfo.faceRect.width / selectBaseImage.imageInfo.width;
            var small_height_ratio = selectBaseImage.faceInfo.faceRect.height / selectBaseImage.imageInfo.height;
            var small_x_ratio = selectBaseImage.faceInfo.faceRect.x / selectBaseImage.imageInfo.width;
            var small_y_ratio = selectBaseImage.faceInfo.faceRect.y / selectBaseImage.imageInfo.height;

            var width = small_width_ratio * drawSize.width;
            var height = small_height_ratio * drawSize.height;
            var x = small_x_ratio * drawSize.width;
            var y = small_y_ratio * drawSize.height;

            var new_width = width * imageScale;
            var new_height = height * imageScale;
            x = x - (new_width - width) / 2;
            y = y - (new_height - height) / 2;

            var trans_x_ratio = imageTransX / imageBoxRect.width;
            var trans_y_ratio = imageTransY / imageBoxRect.height;
            var trans_x = trans_x_ratio * drawSize.width;
            var trans_y = trans_y_ratio * drawSize.height;

            console.log("x,y:" + x + "x" + y + " 宽高:" + width + "x" + height + " 旋转:" + imageRotate + " 缩放:" + imageScale);

            var megicHeadImageData = megicHeadImageInfo.imageData;

            //缩放头像
            if (megicHeadImageInfo.width != new_width || megicHeadImageInfo.height!=new_height){
              wx.showLoading({
                title: '缩放头像',
                mask: true
              });
              getApp().Module["_resize_image"](faceDetectHelper.allocTypedArrayBuffer(megicHeadImageInfo.imageData), megicHeadImageInfo.imageData.length, megicHeadImageInfo.width, megicHeadImageInfo.height, new_width, new_height);
              megicHeadImageData = page.getImageDataFromPtr(getApp().resize_image_result);
              new_width = getApp().resize_image_result.width;
              new_height = getApp().resize_image_result.height;
            }

            //旋转头像
            if (imageRotate!=0.0){
              wx.showLoading({
                title: '旋转头像',
                mask: true
              });
              getApp().Module["_rotate_image"](faceDetectHelper.allocTypedArrayBuffer(megicHeadImageData), megicHeadImageData.length, new_width, new_height, imageRotate * 2);
              megicHeadImageData = page.getImageDataFromPtr(getApp().rotate_image_result);
              new_width = getApp().rotate_image_result.width;
              new_height = getApp().rotate_image_result.height;
            }

            //生成头像png(putImageData不支持半透明颜色混合)
            getApp().Module["_create_png"](
              faceDetectHelper.allocTypedArrayBuffer(megicHeadImageData),
              megicHeadImageData.length,
              new_width,
              new_height
            );
            if (getApp().create_png_result) {
              var image = page.getImageDataFromPtr(getApp().create_png_result);
              let data = new Uint8Array(image);
              //头像保存到临时文件以便canvas绘制
              let fsm = wx.getFileSystemManager();
              let filePath = `${wx.env.USER_DATA_PATH}/` + '' +Date.now()+'_head.png';
              fsm.writeFile({
                filePath: filePath,
                data: data.buffer,
                success: function (res) {
                  wx.showLoading({
                    title: '替换头像',
                    mask: true
                  });
                  // console.log("drawImage:", filePath);
                  canvasContext.drawImage(filePath, x + trans_x, y + trans_y);

                  if (labelText && labelText.trim().length > 0) {
                    wx.showLoading({
                      title: '拼接文字',
                      mask: false
                    });
                    //生成文字图片
                    var rawFontSize = rpx_to_px(page.data.labelTextSize);
                    var scale = imageBoxRect.width / drawSize.width;
                    //console.log("rawFontSize=", rawFontSize, "scale=" + scale);
                    var realLabelLeft = labelX / scale;
                    var realLabelTop = labelY / scale;
                    var realFontSize = rawFontSize / scale;
                    //console.log("realFontSize=", realFontSize);
                    canvasContext.setFontSize(realFontSize);
                    canvasContext.setTextBaseline("top");
                    var textSize = canvasContext.measureText(labelText).width;
                    //console.log("textSize=", textSize);
                    canvasContext.fillStyle = "rgba(0, 0, 0, 0)";
                    canvasContext.fillRect(0, 0, 20, 20);
                    canvasContext.fillStyle = textColor;
                    canvasContext.fillText(labelText, parseInt(realLabelLeft), parseInt(realLabelTop) - rpx_to_px(20));
                  }

                  canvasContext.draw(false, function (res) {
                    wx.showLoading({
                      title: '保存图片',
                      mask: true
                    });

                    wx.canvasToTempFilePath({
                      x: 0,
                      y: 0,
                      width: drawSize.width,
                      height: drawSize.height,
                      destWidth: drawSize.width,
                      destHeight: drawSize.height,
                      fileType: "jpg",
                      canvasId: "scale-canvas",
                      success(res) {
                        fsm.unlink({ filePath: filePath });//删除临时头像文件
                        var tempFilePath = res.tempFilePath;
                        wx.saveImageToPhotosAlbum({
                          filePath: tempFilePath,
                          fail: function (res) {
                            wx.showModal({
                              title: "温馨提示",
                              content: '图片保存失败！',
                              showCancel: false
                            });
                          },
                          success(res) {
                            console.log("已保存到相册:", res);
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
                                    urls: [tempFilePath]
                                  });
                                } else if (res.cancel) {

                                }
                              }
                            });
                          },
                          complete() {
                            wx.hideLoading();
                          }
                        })
                      }
                    });
                  });
                

                    // wx.canvasPutImageData({
                    //   canvasId: "scale-canvas",
                    //   x: labelImage.x,
                    //   y: labelImage.y,
                    //   width: labelImage.width,
                    //   height: labelImage.height,
                    //   data: labelImage.imageData,
                    //   fail(res){
                    //     console.log("文字拼接失败:", res);
                    //   },
                    //   success(res) {
                    //     console.log("文字拼接完成", res);
                    //     saveNext();
                    //     // var imageData = imageInfo.imageData;
                    //     // if (newFinalImageData) {
                    //     //   imageData = newFinalImageData;
                    //     // }

                    //     // getApp().Module["_create_jpeg"](
                    //     //   faceDetectHelper.allocTypedArrayBuffer(imageData),
                    //     //   imageData.length,
                    //     //   imageInfo.width,
                    //     //   imageInfo.height,
                    //     //   1.0//原始大小
                    //     // );
                    //     // if (getApp().create_jpeg_result) {
                    //     //   var data = page.getImageDataFromPtr(getApp().create_jpeg_result);

                    //     //   let fsm = wx.getFileSystemManager();
                    //     //   let filePath = `${wx.env.USER_DATA_PATH}/` + 'painting.jpg';
                    //     //   fsm.writeFile({
                    //     //     filePath: filePath,
                    //     //     data: data.buffer,
                    //     //     success: function (res) {
                    //     //       wx.saveImageToPhotosAlbum({
                    //     //         filePath: filePath,
                    //     //         fail: function (res) { },
                    //     //         success(res) {
                    //     //           // console.log("相册保存成功", res);
                    //     //           //图片已保存到相册
                    //     //           wx.showModal({
                    //     //             title: '保存成功',
                    //     //             content: '图片已保存到相册，请到相册查看，或进入“预览”界面长按分享',
                    //     //             showCancel: true,
                    //     //             cancelText: '关闭',
                    //     //             confirmText: '预览',
                    //     //             success(res) {
                    //     //               if (res.confirm) {
                    //     //                 wx.previewImage({
                    //     //                   urls: [filePath]
                    //     //                 });
                    //     //               } else if (res.cancel) {

                    //     //               }
                    //     //             }
                    //     //           });
                    //     //         }
                    //     //       })
                    //     //     },
                    //     //     fail: function (res) {
                    //     //       console.log("图片保存失败", res);
                    //     //       wx.showModal({
                    //     //         title: "温馨提示",
                    //     //         content: '图片保存失败！',
                    //     //         showCancel: false
                    //     //       });
                    //     //     },
                    //     //     complete: function () {
                    //     //       wx.hideLoading();
                    //     //     }
                    //     //   });
                    //     // } else {
                    //     //   wx.showModal({
                    //     //     title: "温馨提示",
                    //     //     content: "jpeg生成失败，请退出重试。",
                    //     //     showCancel: false
                    //     //   });
                    //     //   wx.hideLoading();
                    //     // }
                    //   }
                    // });
                      // try {
                      //   if (labelImage) {
                      //     wx.showLoading({
                      //       title: '拼接文字',
                      //       mask: true
                      //     });

                      //     getApp().Module["_draw_image"](
                      //       faceDetectHelper.allocTypedArrayBuffer(imageInfo.imageData),
                      //       imageInfo.imageData.length,
                      //       imageInfo.width,
                      //       imageInfo.height,

                      //       faceDetectHelper.allocTypedArrayBuffer(labelImage.imageData),
                      //       labelImage.imageData.length,
                      //       labelImage.width,
                      //       labelImage.height,
                      //       labelImage.x,
                      //       labelImage.y,
                      //       labelImage.width,
                      //       labelImage.height,
                      //       0.0
                      //     );
                      //     if (getApp().draw_image_result) {
                      //       newFinalImageData = page.getImageDataFromPtr(getApp().draw_image_result);
                      //       //console.log("文字拼接", x + "," + y + " " + width + "," + height, finalImageInfo);
                      //     } else {
                      //       console.log("文字拼接失败");
                      //     }
                      //   }
                      // } catch (e) {
                      //   console.log("文字拼接失败", e);
                      // }
                },
                fail: function (res) {
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
            } else {
              wx.hideLoading();
              wx.showModal({
                title: "温馨提示",
                content: "png生成失败，请退出重试。",
                showCancel: false
              });
            }

            // getApp().Module["_draw_image"](
            //   faceDetectHelper.allocTypedArrayBuffer(imageInfo.imageData),
            //   imageInfo.imageData.length,
            //   imageInfo.width,
            //   imageInfo.height,
            //   //头像数据
            //   faceDetectHelper.allocTypedArrayBuffer(megicHeadImageInfo.imageData),
            //   megicHeadImageInfo.imageData.length,
            //   megicHeadImageInfo.width,
            //   megicHeadImageInfo.height,
            //   x + trans_x, //x坐标
            //   y + trans_y, //y坐标
            //   new_width,
            //   new_height,
              
            // );
            // if (getApp().draw_image_result) {
            //   newFinalImageData = page.getImageDataFromPtr(getApp().draw_image_result);
            //   //console.log("头像拼接", x + "," + y + " " + width + "," + height, imageInfo);
            // } else {
            //   console.log("头像拼接失败");
            // }
          });
          savedPhoto = true;
          wx.setStorage({
            key: "savedPhoto",
            data: true
          });
        };

        //获取相册授权
        wx.getSetting({
          success(res) {
            if (!res.authSetting['scope.writePhotosAlbum']) {
              wx.openSetting({
              });
            } else {
              save();
            }
          }
        });
      } else {
        wx.hideLoading();
        wx.showModal({
          title: "温馨提示",
          content: '图片读取失败！',
          showCancel: false
        });
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
            if (res.tempFilePaths[0] != selectBaseImage.localPath){
              selectBaseImage.localPath = res.tempFilePaths[0];
              selectBaseImage.name = app.getFileName(res.tempFilePaths[0]);
              selectBaseImage.dbFaceRect = null;
              selectBaseImage.smallPath = res.tempFilePaths[0];
              selectBaseImage.bigPath = res.tempFilePaths[0];
              selectBaseImage.colorType = "0";
              page.updateBaseImage(true);
            }
          }else{
            var choosePath = res.tempFilePaths[0];
            wx.showLoading({
              title: '读取图片',
              mask: false
            });
            page.loadFrontImage(choosePath, function () {
              if (frontImage.imageInfo != null) {
                page.setData({ showUserHeadImage: false, currentFrontImageName: app.getFileName(choosePath), currentFrontImagePath: choosePath },
                  function () {
                    page.setData({ imageTransX: 0.0, imageTransY: 0.0, dtRotateValue: 60, dtScaleValue: 30 });
                    imageScale = 1.0;
                    imageRotate = 0.0;
                    imageTransX = 0.0;
                    imageTransY = 0.0;
                    page.replaceHead();
                  });
              } else{
                wx.hideLoading();
              }
            }, true);
          }
        }
      }
    });
  },

  onClose: function(){
    nextShowTime = 0;
  },

  onShow: function(){
    console.log("nextShowTime=", nextShowTime, "interstitialAd=", interstitialAd);
    //1分钟显示一次广告
    if (savedPhoto && new Date().getTime() > nextShowTime) {
      if (interstitialAd) {
        nextShowTime = new Date().getTime() + 1000 * 60;
        interstitialAd.show().catch((err) => {
          nextShowTime -= 1000 * 60;
          console.error(err)
        });
      }
    } else {
      if (!savedPhoto) {
        console.log("用户未使用，不显示广告");
      } else {
        console.log("时间未到，不显示广告");
      }
    }
  },

  onLoad: function (options) {
    // 在页面onLoad回调事件中创建插屏广告实例
    if (wx.createInterstitialAd) {
      interstitialAd = wx.createInterstitialAd({
        adUnitId: 'adunit-a0a266b12ccbff68'
      })
      interstitialAd.onLoad(() => { })
      interstitialAd.onError((err) => { })
      interstitialAd.onClose(() => { });
      console.log("interstitialAd已经初始化", interstitialAd);
    } else {
      console.log("wx.createInterstitialAd不存在");
    }
    try {
      try {
        var value = wx.getStorageSync('savedPhoto')
        if (value) {
          savedPhoto = true;
        }
      } catch (e) {
        console.log("配置文件读取失败", e);
      }
    } catch (e) { }

    canvasContext = wx.createCanvasContext("scale-canvas");
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

    getApp().page = this;
    var page = this;
    wx.showLoading({
      title: '正在启动',
      mask: false
    });
    getApp().t = Date.now();

    var page = this;

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

    this.setData({
      // convertImageData: getApp().globalData.imageListCol0[0].small,
      imageListCol0: getApp().globalData.imageListCol0,
      imageListCol1: getApp().globalData.imageListCol1,
    });

    console.log("初始化成功:" + (Date.now() - getApp().t) + "ms");
    // wx.hideLoading();
  },

  //调用asmjs函数
  // callLibrary(func, params){
  //   console.log("callLibrary", func, params);
  //   var jsString = params;
  //   var lengthBytes = getApp().Module.lengthBytesUTF8(jsString) + 1;
  //   var stringOnWasmHeap = getApp().Module._malloc(lengthBytes);
  //   getApp().Module.stringToUTF8(jsString, stringOnWasmHeap, lengthBytes);
  //   getApp().Module["_" + func](stringOnWasmHeap);
  // },

  loadImage: function(path, cb){
    faceDetectHelper.compressImage(1200.0, 2000.0, path, canvasContext, function(res){
      cb({
        imageData: res.data,
        width: res.width,
        height: res.height
      });
    });
  },

  loadSmallImage: function (path, cb, userChoose) {
    faceDetectHelper.compressImage(460.0, 460.0, path, canvasContext, function (res) {
      console.log("压缩图片返回:", res);
      if(res){
        cb({
          imageData: res.data,
          width: res.width,
          height: res.height
        });
      }else{
        cb();
      }
    }, userChoose);
  },

  //读取前景图
  //userChoose 是否为用户选择的图片，用户选择的图片要进行验证
  loadFrontImage: function(path, cb, userChoose){
    frontImage.localPath = path;
    var page = this;
    this.loadSmallImage(path, function(data){
      if(!data){
        cb();
        return;
      }
      frontImage.imageInfo = data;

      //提取用户头像
      wx.showLoading({
        title: '识别人脸...',
        mask: true,
      });
      page.detectFace(frontImage, function (frontFaceInfo){
        frontImage.faceInfo = frontFaceInfo;
        wx.hideLoading();
        cb();
      });
    }, userChoose);
  },

  //读取底图
  loadBaseImage: function (path, cb, userChoose){
    this.setData({ showLabelTip: false, showUserHeadImage: false});
    var page = this;
    this.loadSmallImage(path, function (data) {
      if(!data){
        cb(true);
        return;
      }
      selectBaseImage.imageInfo = data;
      //提取底图人脸
      wx.showLoading({
        title: '识别人脸...',
        mask: true,
      });
      page.detectFace(selectBaseImage, function (baseFaceInfo){
        if (!baseFaceInfo && page.data.currentFrontImagePath == DEFAULT_HEAD_PATH) {
          wx.showModal({
            title: '温馨提示',
            content: '底图中未检测到人脸，请重新选择！' + NO_FACE_TIP,
            showCancel: false,
          });
        }
        wx.hideLoading();

        selectBaseImage.faceInfo = baseFaceInfo;
        console.log("底图人脸:", baseFaceInfo);
        page.setImageBoxRect();
        cb();
      });
    }, userChoose);
  },

  initOnce: function(){
    // console.log("调用 initOnce>>>");
    //初始化rust_face
    getApp().t = Date.now();
    wx.showLoading({
      title: '初始化中...',
      mask: true
    });
    getApp().Module["_init_detector"]();
    console.log("人脸识别库读取完成:" + (Date.now() - getApp().t) + "ms");

    //显示读取的第一张图
    this.selectPresetImage({
      currentTarget: {
        dataset: {
          id: 0,
          list: 0,
        }
      }
    }, function () {
      //稍后读取数据
      imageLoader.pullImages();
    });
    init = true;
  },

  onSmallBaseImageLoad: function(e){
    smallBaseLoad = true;
    if (smallBaseLoad && canvasLoad){
      if (init) {
        wx.hideLoading();
        return;
      }
      this.initOnce();
    }
  },

  //图片显示完成再进行初始化
  onCanvasImageLoad: function(e){
    canvasLoad = true;
    if (smallBaseLoad && canvasLoad) {
      if (init) {
        wx.hideLoading();
        return;
      }
      this.initOnce();
    }
  },

  //修改色阶 (默认40/64)
  bindColorLevelChange: function (e) {
    //只有底图和前景都存在的情况下，选择色阶才起作用
    //console.log("bindColorLevelChange>>>", e.detail.value);
    if (selectBaseImage.imageInfo && this.data.currentFrontImagePath != DEFAULT_HEAD_PATH) {
      this.replaceHead(colorLevelsArray[parseInt(e.detail.value)]);
    }
  },

  //-------------- 输入文字相关 ---------------
  bindTextColorChange: function (res) {
    textColor = textColors[res.detail.value];
    this.setData({ labelColor:textColor, textColor: this.data.textColorArray[res.detail.value], textColorId: res.detail.value });
  },
  showInputText: function () {
    this.setData({ isInputTextHidden: false });
  },
  labelTextSizeChange: function(e){
    this.setData({ labelTextSize: e.detail.value});
  },
  bindText: function (e) {
    bindText = e.detail.value;
  },
  setText: function () {
    labelText = bindText;
    this.setData({ labelText: labelText, isInputTextHidden: true });
  },
  clearText: function () {
    labelText = "";
    labelImage = null;
    this.setData({ labelText: '输入文字' });
    this.setData({ isInputTextHidden: true, showLabel:false, });
  },

  getConvertImageData: function(){
    var convertImageData = page.data.convertImageData1 + page.data.convertImageData2 + page.data.convertImageData3;
    return wx.base64ToArrayBuffer(convertImageData.replace("data:image/gif;base64,", ""));
  },

  writeJpeg: function (data, cb) {
    wx.showLoading({
      title: '写入文件',
      mask: true,
    });
    let fsm = wx.getFileSystemManager();
    let filePath = `${wx.env.USER_DATA_PATH}/` + 'painting.jpg';
    //console.log("写入jpeg", filePath);
    fsm.writeFile({
      filePath: filePath,
      data: data,
      success: function (res) {
        cb(filePath);
      },
      fail: function (res) {
        cb();
      }
    });
  },

  getImageDataFromPtr(imageInfo){
    let imageDataPtr = imageInfo.imageDataPtr;
    let imageDataLen = imageInfo.imageDataLen;
    var imageData = new Uint8ClampedArray(getApp().Module.HEAPU8.subarray(imageDataPtr, imageDataPtr + imageDataLen).slice(0));
    //rust中使用 mem::forget传递过来的指针，要在asmjs中手动释放
    getApp().Module._free(imageDataPtr);
    return imageData;
  },

  //提取人脸位置和图片数据
  //localPath(用于获取缓存)
  detectFace: function(image, callback) {
    var imageInfo = image.imageInfo;
    var bigPath = image.bigPath || image.localPath; //检测大图人脸

    var face_rect;
    if (image.dbFaceRect){
      face_rect = image.dbFaceRect;
      console.log("预置人脸区域信息:", face_rect);
    }else{
      face_rect = faceDetectHelper.detectFace(imageInfo.imageData, imageInfo.width, imageInfo.height, 100);
    }
    if (!face_rect){
      callback(null);
      return;
    }

    console.log("人脸", face_rect);

    //人脸区域不再用js截取，将大图在canvas中截取
    // getApp().Module["_sub_image"](faceDetectHelper.allocTypedArrayBuffer(imageInfo.imageData), imageInfo.imageData.length, imageInfo.width, imageInfo.height, face_rect.x, face_rect.y, face_rect.width, face_rect.height);

    // var imageInfo = {
    //   imageData: this.getImageDataFromPtr(getApp().sub_image_result),
    //   width: getApp().sub_image_result.width,
    //   height: getApp().sub_image_result.height
    // };

    //在canvas中截取大图的人脸头像
    var width = imageInfo.width;
    var height = imageInfo.height;
    var x_ratio = face_rect.x / width;
    var y_ratio = face_rect.y / height;
    var width_ratio = face_rect.width / width;
    var height_ratio = face_rect.height/ height;
    imageLoader.getBigImagePath(bigPath, function (path) {
      faceDetectHelper.getSubImage(1200, 1200, path, x_ratio, y_ratio, width_ratio, height_ratio, canvasContext, function (imageInfo) {
        console.log("大图人脸头像:", imageInfo);
        callback({
          imageInfo: {
            imageData: imageInfo.data,
            width: imageInfo.width,
            height: imageInfo.height,
          },
          faceRect: face_rect
        });
      });
    });
  }
})
/*

1、选择前景图的时候，验证图片是否合规，即 loadFrontImage 方法中
2、选择底图的时候，验证图片是否合规，即 updateBaseImage 方法中
3、以上两个方法都调用了 loadSmallImage 方法，所以在此方法中验证就可以。
4、loadSmallImage调用faceDetectHelper.compressImage，所以在faceDetectHelper.compressImage对图片进行验证
 */