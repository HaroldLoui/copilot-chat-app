<template>
  <div class="app-container">
    <div class="app-sidebar">
      <div class="sidebar-header">
        <div class="sidebar-header-title">HaroldLoui's Chat App</div>
        <div class="sidebar-header-sub-title">个人聊天应用</div>
      </div>
      <div class="sidebar-main">
        <template v-if="chatList.length > 0">
          <chat-box v-for="cb in chatList" :key="cb.id" :value="cb" @close="onClose"></chat-box>
        </template>
        <chat-box v-else :value="blankChat" @close="onClose"></chat-box>
      </div>
      <div class="sidebar-footer"></div>
    </div>
    <div class="app-main"></div>
  </div>
</template>

<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

import { reactive, ref } from "vue";
import ChatBox from "./components/ChatBox.vue";

const getNow = () => {
  const date = new Date();
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  const hours = String(date.getHours()).padStart(2, "0");
  const minutes = String(date.getMinutes()).padStart(2, "0");
  const seconds = String(date.getSeconds()).padStart(2, "0");

  return `${year}/${month}/${day} ${hours}:${minutes}:${seconds}`;
};

const blankChat = reactive<ChatBox>({
  id: "",
  title: "默认对话",
  count: 0,
  createTime: getNow(),
});
const chatList = ref<ChatBox[]>([]);

const onClose = (value: boolean) => {
  if (value) {
    console.log("删除成功,更新列表");
  } else {
    blankChat.createTime = getNow();
  }
};
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
</script>

<style>
body {
  margin: 0;
  padding: 0;
}

#app {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #fafafa;
}
</style>

<style scoped lang="scss">
.app-container {
  min-width: 600px;
  min-height: 370px;
  max-width: 1200px;
  width: 90vw;
  height: 90vh;
  background-color: #f0f0f0;
  border: 1px solid #dedede;
  border-radius: 20px;
  box-shadow: 50px 50px 100px 10px rgba(0, 0, 0, 0.1);
  box-sizing: border-box;
  overflow: hidden;

  .app-sidebar {
    width: 283px;
    height: 100%;
    padding: 20px;
    display: flex;
    flex-direction: column;
    background-color: #e7f8ff;
    box-shadow: inset -2px 0 2px 0 rgba(0, 0, 0, 0.05);
    box-sizing: border-box;

    .sidebar-header {
      height: 60px;
      padding-left: 5px;

      .sidebar-header-title {
        height: 50%;
        font-size: 20px;
        font-weight: bolder;
      }

      .sidebar-header-sub-title {
        height: 50%;
        font-size: 14px;
        color: #686868;
      }
    }

    .sidebar-main {
      flex: 1;
      margin: 10px 0;
      overflow-y: auto;
      overflow-x: hidden;
      padding: 10px;

      &::-webkit-scrollbar {
        width: 0;
      }
    }

    .sidebar-footer {
      height: 60px;
      background-color: blue;
    }
  }

  .app-main {
    width: calc(100% - 283px);
    height: 100%;
  }
}
</style>
