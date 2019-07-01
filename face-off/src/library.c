#include "F:\emsdk\fastcomp\emscripten\system\include\emscripten\emscripten.h"
// #include "D:\emsdk1.38\emsdk\emscripten\1.38.24\system\include\emscripten\emscripten.h"

//EM_JS用法 参考 https://emscripten.org/docs/api_reference/emscripten.h.html

// EM_JS(void, test_on_message, (), {
//   var msg = JSON.stringify({
//         text: 'hello! 哈哈!',
//         width: 100,
//         height: 200,
//     });
//     var ptr = allocateUTF8OnStack(msg);
//     console.log(ptr);
//     Module['_on_message'](ptr);
//     //_free
//     //stackRestore(ptr);
// });

//alert
EM_JS(void, _js_show_modal, (const char* str), {
    var msg = UTF8ToString(str);
    console.log("js_show_modal", msg);
    wx.showModal({
      title: '温馨提示',
      content: msg,
      showCancel: false,
    });
});

//进度对话框
EM_JS(void, _js_show_loading, (const char* str), {
  wx.showLoading({
        title: UTF8ToString(str),
        mask: true
    });
});

EM_JS(void, _js_set_app_field, (const char* key, const char* value), {
  getApp()[UTF8ToString(key)] = UTF8ToString(value);
});

EM_JS(void, _js_set_app_field_json, (const char* key, const char* value), {
  getApp()[UTF8ToString(key)] = JSON.parse(UTF8ToString(value));
});

EM_JS(void, _js_delete_app_field, (const char* key), {
  getApp()[UTF8ToString(key)] = null;
});

EM_JS(void, _js_set_module, (), {
  getApp().Module = Module;
  getApp().Module.allocateUTF8OnStack = allocateUTF8OnStack;
  getApp().Module.lengthBytesUTF8 = lengthBytesUTF8;
  getApp().Module.stringToUTF8 = stringToUTF8;
});

//获取cascade文件的buffer数据
EM_JS(char*, _js_get_cascade_file_data, (), {
//   // 'jsString.length' would return the length of the string as UTF-16
//   // units, but Emscripten C strings operate as UTF-8.
//   var lengthBytes = lengthBytesUTF8(jsString)+1;
//   var stringOnWasmHeap = _malloc(lengthBytes);
//   stringToUTF8(jsString, stringOnWasmHeap, lengthBytes);
//   return stringOnWasmHeap;

    //getApp().globalData.FILE_DATA: ArrayBuffer
    // console.log("getApp().globalData.FILE_DATA=", getApp().globalData.FILE_DATA);
    var data = new Uint8Array(getApp().globalData.FILE_DATA);
    var buf = Module._malloc(data.length * data.BYTES_PER_ELEMENT);
    Module.HEAPU8.set(data, buf);
    console.log('FILE_DATA Buffer 长度:' + data.length+"指针:"+buf);
    return buf;
});

EM_JS(int, _js_get_cascade_file_data_size, (), {
    return getApp().globalData.FILE_DATA.byteLength;
});

void js_set_module(){
    _js_set_module();
}

void js_set_app_field(const char* key, const char* value){
    _js_set_app_field(key, value);
}

void js_set_app_field_json(const char* key, const char* value){
    _js_set_app_field_json(key, value);
}

void js_show_modal(const char* msg){
    _js_show_modal(msg);
}

void js_delete_app_field(const char* key){
    _js_delete_app_field(key);
}

char* js_get_cascade_file_data(){
    return _js_get_cascade_file_data();
}
int js_get_cascade_file_data_size(){
    return _js_get_cascade_file_data_size();
}

void js_show_loading(const char* str){
    _js_show_loading(str);
}

double _current_timestamp() {
    double timestamp = EM_ASM_DOUBLE({
        return parseFloat(Date.now());
    });
    return timestamp;
}