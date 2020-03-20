var fs = require("fs");  

var code = fs.readFileSync("./target/asmjs-unknown-emscripten/release/face-off.js", "utf-8");

//注释掉两个 Module["arguments"]=arguments
var code = code.replace('Module["arguments"]=arguments', '');
// //new_content = new_content.replace('ENVIRONMENT_IS_WORKER=typeof importScripts==="function";', '');
// //new_content = new_content.replace('Module["arguments"]=arguments', '');
// code = code.replace('FS.staticInit();__ATINIT__.unshift((function(){if(!Module["noFSInit"]&&!FS.init.initialized)FS.init()}));__ATMAIN__.push((function(){FS.ignorePermissions=false}));__ATEXIT__.push((function(){FS.quit()}));__ATINIT__.unshift((function(){TTY.init()}));__ATEXIT__.push((function(){TTY.shutdown()}));', '');

//解决：str.charCodeAt is not a function
code = code.replace('ENVIRONMENT_IS_WEB=typeof window==="object";', 'ENVIRONMENT_IS_WEB=true;var document={};');

//处理图像加大内存(2的整数倍)
code = code.replace('Module["TOTAL_STACK"]||5242880', 'Module["TOTAL_STACK"]||parseInt(5242880*6)');//栈内存总共30M
code = code.replace('Module["TOTAL_MEMORY"]||16777216', 'Module["TOTAL_MEMORY"]||parseInt(16777216*6)');//总共96M
// code = code.replace('ENVIRONMENT_IS_WORKER=typeof importScripts==="function";', 'ENVIRONMENT_IS_WORKER=false;');

// code = code.replace('Runtime.dynCall', 'dynCall');

fs.writeFileSync("../miniprogram/faceoff/pages/index/face-off.js", code);