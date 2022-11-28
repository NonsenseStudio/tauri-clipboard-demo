<script setup lang="ts">

// 用于获取应用名称、Tauri版本号和应用版本号
import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app'

// 导入系统剪切板 Api 接口函数
import { readText, writeText } from '@tauri-apps/api/clipboard';

import { Ref, ref, onMounted } from "vue";

// 从 Bootstrap5 中导入 Toast 提示框模块
import { Toast } from 'bootstrap';



interface AppInfo {
  appName?: string,
  appVersion?: string,
  tauriVersion?: string
}

const appInfo: Ref<AppInfo> = ref({});



// 获取 appName 
getName().then((v) => { appInfo.value.appName = v });
// 打印 Tauri 版本
getTauriVersion().then((v) => { appInfo.value.tauriVersion = v });
// App Version
getVersion().then((v) => { appInfo.value.appVersion = v });


const clipboard = ref("");

// 获取剪切板内容，并通过 Bootstrap Toast 组件显示出来
function getClipboardValue() {

  readText().then((v) => { if (v != null) { clipboard.value = v; } });

  const clipboard_element = document.querySelector("#clipboard");
  if (clipboard_element != null) {
    new Toast(clipboard_element, { autohide: true, delay: 3000 }).show(); // autohide 控制提示窗口是否自动隐藏
  }
}

// testText 关联到文本框，并且写入初始值
const testText = ref("Hello,World!");

// 把文本框中的内容复制的系统剪切板中
function copyText() {
  writeText(testText.value);
}

// 清空系统剪切板
function cleanClipboard() {
  writeText("");
}

// onMounted 钩子可以用来在组件完成初始渲染并创建 DOM 节点后运行代码
onMounted(() => {

  // 初始化所有 .toast 元素
  // Array.from(document.querySelectorAll(".toast"))
  //   .forEach(toastNode => new Toast(toastNode))
})


</script>

<template>
  <div class="container">
    <h1 class="text-center my-5">Hello,Tauri!</h1>
    <div class="row">

      <div class="col-md-4 col-sm-12">
        <h2>程序信息</h2>
        <ul>
          <li>App Name: <span class="badge bg-primary">{{ appInfo?.appName }}</span></li>
          <li>App Version: <span class="badge bg-primary">{{ appInfo?.appVersion }}</span></li>
          <li>Tauri version: <span class="badge bg-primary">{{ appInfo?.tauriVersion }}</span></li>
        </ul>
      </div>

      <div>
        <div class="btn-group mb-2" role="group" aria-label="Basic outlined example">
          <button type="button" class="btn btn-outline-primary" @click="copyText">复制输入框中的内容到剪切板</button>
          <button type="button" class="btn btn-outline-primary" @click="getClipboardValue">显示剪切板内容</button>
          <button type="button" class="btn btn-outline-primary" @click="cleanClipboard">清空剪切板</button>
        </div>
      </div>
      <textarea type="text" rows="6" v-model="testText"></textarea>


      <div id="clipboard" class="toast fs-5 position-fixed bottom-0 end-0 p-0 me-1 mb-1" role="alert" aria-live="assertive"
        aria-atomic="true">
        <div class="toast-header text-bg-primary border-0">
          <i class="bi bi-ubuntu me-2"></i> <strong class="me-auto">剪切板中的内容为</strong>
          <small>自动隐藏</small>
        </div>
        <div class="toast-body custom-toast-body">
          {{ clipboard }}
        </div>
      </div>


    </div>
  </div><!-- container END -->
</template>

<style scoped>
.custom-toast-body {
  background: #ddd;
}
</style>
