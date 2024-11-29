<template>
  <div class="siderbar-header">
    <div class="title">Chat App</div>
    <div class="search">
      <vs-input style="width: 100%" state="primary" v-model="searchValue" icon-after placeholder="搜索聊天...">
        <template #icon>
          <i class="bx bx-search"></i>
        </template>
      </vs-input>
    </div>
  </div>
  <div class="siderbar-main">
    <div class="chat-list">
      <ChatBox active></ChatBox>
      <ChatBox v-for="i in 20" :key="i"></ChatBox>
    </div>
  </div>
  <div class="siderbar-footer">
    <div class="btns">
      <vs-button type="flat">
        <i class="bx bx-cog"></i>
        <template #animate> 设置 </template>
      </vs-button>
      <vs-button type="flat">
        <i class="bx bxl-github"></i>
        <template #animate> 源码 </template>
      </vs-button>
      <vs-button type="flat" color="success" animation-type="scale" style="width: 100px">
        <i class="bx bx-plus"></i>
        <template #animate> 添加聊天 </template>
      </vs-button>
    </div>
  </div>
</template>

<script setup lang="ts">
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

import { onMounted, reactive, ref } from "vue";
import ChatBox from "../components/ChatBox.vue";

onMounted(() => {
  onSearchChat();
  console.log(chatList, blankChat);
});

const searchValue = ref<string>("");
const onSearchChat = () => {
  console.log(searchValue.value);
};

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
</script>

<style lang="scss" scoped>
.siderbar-header {
  height: 100px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;

  .title {
    height: 60px;
    line-height: 60px;
    text-align: center;
    font-size: 30px;
    font-weight: bolder;
  }

  .search {
    width: 90%;
  }
}

.siderbar-main {
  flex: 1;
  overflow-x: hidden;
  overflow-y: auto;

  .chat-list {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
  }

  &::-webkit-scrollbar {
    width: 0;
  }
}

.siderbar-footer {
  height: 60px;

  .btns {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-around;
  }
}
</style>
