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
      <ChatBox
        v-for="(chat, i) in chatList"
        :key="i"
        :value="chat"
        :active="activeIndex === i"
        @click="handleChooseChat(i, chat)"
        @delete="handleDeleteChat"
      ></ChatBox>
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
      <vs-button type="flat" color="success" animation-type="scale" style="width: 100px" @click="handleAddChat">
        <i class="bx bx-plus"></i>
        <template #animate> 添加聊天 </template>
      </vs-button>
    </div>
  </div>

  <vs-dialog v-model="deleteChatDialog" width="550px" not-center>
    <template #header>
      <h6 class="not-margin"></h6>
    </template>

    <div class="con-content">
      <p>删除后不可恢复！确认删除该对话？</p>
    </div>

    <template #footer>
      <div class="con-footer">
        <vs-button type="transparent" @click="invokeDeleteChat"> 确认 </vs-button>
        <vs-button color="danger" type="transparent" @click="deleteChatDialog = false"> 取消 </vs-button>
      </div>
    </template>
  </vs-dialog>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import ChatBox from "../components/ChatBox.vue";

onMounted(() => {
  onSearchChat();
});

const searchValue = ref<string>("");
const onSearchChat = () => {
  console.log(searchValue.value);
  getChatList();
};

const chatList = ref<ChatBox[]>([]);
const activeIndex = ref<number>(0);
const getChatList = async () => {
  chatList.value = await invoke("list_chat_box_api", { title: searchValue.value });
};

const handleAddChat = async () => {
  await invoke("insert_chat_box_api");
  getChatList();
};

const emits = defineEmits(["changeChat"]);
const handleChooseChat = (i: number, chat: ChatBox) => {
  activeIndex.value = i;
  emits("changeChat", chat);
};

const deleteChatDialog = ref<boolean>(false);
const deleteChatId = ref<string>("");
const handleDeleteChat = (id: string | number) => {
  deleteChatId.value = id.toString();
  deleteChatDialog.value = true;
};
const invokeDeleteChat = async () => {
  await invoke("delete_chat_box_api", { id: deleteChatId.value });
  getChatList();
  deleteChatDialog.value = false;
};
</script>

<style lang="scss" scoped>
.siderbar-header {
  height: 100px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;

  .title {
    height: 50px;
    line-height: 50px;
    text-align: center;
    font-size: 28px;
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

.not-margin {
  margin: 0px;
  font-weight: normal;
  padding: 10px;
}

.con-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  width: calc(100%);
  .new {
    margin: 0px;
    margin-top: 20px;
    padding: 0px;
    font-size: 0.7rem;
    a {
      color: rgba(var(--vs-primary)) !important;
      margin-left: 6px;
      &:hover {
        text-decoration: underline;
      }
    }
  }
  .vs-button {
    margin: 0px;
  }
}
</style>
