//app.js
App({
  onLaunch: function () {
    wx.cloud.init({
      env: 'pub-5f9af0'
    });
  },
  getFileName: function(path){
    let idx = path.lastIndexOf("/");
    let pidx = path.lastIndexOf(".");
    var name = path.substring(idx + 1, pidx);
    return name.replace("_", ' ');
  },

  globalData: {
    userDataPath: `${wx.env.USER_DATA_PATH}/`
  }
})