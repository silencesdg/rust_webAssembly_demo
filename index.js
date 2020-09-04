// import * as wasm from "./pkg/rw";

// js
document.getElementById("fileImport").addEventListener("click", function () {
  document.getElementById("files").click();
});

window.fileImport = function () {
  //获取读取我文件的 File 对象
  var selectedFile = document.getElementById("files").files[0];
  var reader = new FileReader(); // 这是核心, 读取操作就是由它完成.
  reader.readAsArrayBuffer(selectedFile); // 读取文件的内容,也可以读取文件的URL
  reader.onload = function () {
    var uint8Array = new Uint8Array(this.result);
    import('./pkg/rw').then(res => {
        res.grayscale(uint8Array);
    })
  };
};
