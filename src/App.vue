<template>
  <div class="app-container">
    <div class="app-sidebar">
      <div class="sidebar-header">
        <div class="sidebar-header-title">HaroldLoui's Chat App</div>
        <div class="sidebar-header-sub-title">个人聊天应用</div>
        <div class="sidebar-header-search">
          <input v-model="searchValue" class="search-input" type="text" placeholder="搜索聊天..." />
          <button class="search-button" @click="onSearchChat">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
              <path
                fill="currentColor"
                d="m795.904 750.72 124.992 124.928a32 32 0 0 1-45.248 45.248L750.656 795.904a416 416 0 1 1 45.248-45.248zM480 832a352 352 0 1 0 0-704 352 352 0 0 0 0 704"
              ></path>
            </svg>
          </button>
        </div>
      </div>
      <div class="sidebar-main">
        <template v-if="chatList.length > 0">
          <chat-box v-for="cb in chatList" :key="cb.id" :value="cb" @close="onBoxClose" @click="onChooseChat(cb)">
          </chat-box>
        </template>
        <chat-box v-else :value="blankChat" @close="onBoxClose" @click="onChooseChat(blankChat)"></chat-box>
      </div>
      <div class="sidebar-footer">
        <div class="button-group">
          <button @click="onSetting">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              width="16"
              height="16"
              fill="none"
            >
              <defs><path id="settings_svg__a" d="M0 0h16v16H0z"></path></defs>
              <g>
                <mask id="settings_svg__b" fill="#fff"><use xlink:href="#settings_svg__a"></use></mask>
                <g mask="url(#settings_svg__b)">
                  <path
                    d="M13.33 5.67 10 0H3.33L0 5.67l3.33 5.66H10l3.33-5.66Z"
                    transform="translate(1.333 2.333)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M3.33 1.67C3.33.75 2.59 0 1.67 0 .75 0 0 .75 0 1.67c0 .92.75 1.66 1.67 1.66.92 0 1.66-.74 1.66-1.66Z"
                    transform="translate(6.333 6.333)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                </g>
              </g>
            </svg>
          </button>
          <button @click="onGithub">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              width="16"
              height="16"
              fill="none"
            >
              <defs><path id="github_svg__a" d="M0 0h16v16H0z"></path></defs>
              <g>
                <mask id="github_svg__b" fill="#fff"><use xlink:href="#github_svg__a"></use></mask>
                <g mask="url(#github_svg__b)">
                  <path
                    d="M7.11 8.51c.81-.16 1.53-.45 2.1-.87.96-.73 1.46-1.85 1.46-2.95 0-.78-.3-1.5-.81-2.11-.28-.34.55-2.89-.19-2.55-.73.34-1.81 1.1-2.38.94C6.68.79 6.02.69 5.33.69c-.6 0-1.17.07-1.71.21-.79.2-1.53-.54-2.29-.87C.58-.29.99 2.34.77 2.62.28 3.22 0 3.93 0 4.69c0 1.1.6 2.22 1.56 2.95.65.48 1.45.78 2.35.94"
                    transform="translate(2.667 1.645)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M.58 0C.19.43 0 .83 0 1.21v2.91"
                    transform="translate(6 10.22)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M0 0c.37.48.55.91.55 1.29v2.89"
                    transform="translate(9.782 10.159)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M0 0c.3.04.52.17.67.41C.88.77 1.69 2.1 2.61 2.1H4"
                    transform="translate(2 10.405)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                </g>
              </g>
            </svg>
          </button>
        </div>
        <div style="width: 40px"></div>
        <div class="button-group">
          <button class="add-chat-btn" @click="onAddChat">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              width="16"
              height="16"
              fill="none"
            >
              <defs><path id="add_svg__a" d="M0 0h16v16H0z"></path></defs>
              <g>
                <mask id="add_svg__b" fill="#fff"><use xlink:href="#add_svg__a"></use></mask>
                <g mask="url(#add_svg__b)">
                  <path
                    d="M13.33 6.67A6.66 6.66 0 0 0 6.67 0C2.98 0 0 2.98 0 6.67a6.66 6.66 0 0 0 6.67 6.66c3.68 0 6.66-2.98 6.66-6.66Z"
                    transform="translate(1.333 1.333)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M0 0v5.33"
                    transform="translate(8 5.333)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M0 0h5.33"
                    transform="translate(5.333 8)"
                    style="stroke: rgb(51, 51, 51); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                </g>
              </g>
            </svg>
            <div>添加聊天</div>
          </button>
        </div>
      </div>
    </div>
    <div class="app-main">
      <chat-room :cb="currentChat"></chat-room>
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
import ChatBox from "./components/ChatBox.vue";
import ChatRoom from "./components/ChatRoom.vue";
import { open } from "@tauri-apps/plugin-shell";

onMounted(() => {
  document.addEventListener("DOMContentLoaded", () => {
    document.querySelectorAll("a").forEach((link) => {
      link.addEventListener("click", (e) => {
        e.preventDefault();
        const url = link.getAttribute("href");
        if (isUrl(url)) {
          open(url!);
        }
      });
    });
  });
});

const isUrl = (url: string | null) => {
  if (!url) {
    return false;
  }
  return url.startsWith("https://") || url.startsWith("http://");
};

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

const onBoxClose = (value: boolean) => {
  if (value) {
    console.log("删除成功,更新列表");
  } else {
    blankChat.createTime = getNow();
  }
};

const currentChat = ref<ChatBox>({
  ...blankChat,
});
const onChooseChat = (cb: ChatBox) => {
  currentChat.value = cb;
};

const onSetting = () => {
  console.log("设置");
};

const onGithub = () => {
  console.log("Github");
};

const onAddChat = () => {
  console.log("添加聊天");
};
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
  display: flex;

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
      height: 100px;
      padding-left: 5px;

      .sidebar-header-title {
        height: 30px;
        font-size: 20px;
        font-weight: bolder;
      }

      .sidebar-header-sub-title {
        height: 30px;
        font-size: 14px;
        color: #686868;
      }

      .sidebar-header-search {
        width: 215px;
        height: 40px;
        padding: 0 0.5em;
        display: flex;
        align-items: center;
        justify-content: space-between;
        background-color: #fff;
        border-radius: 8px;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        transition: all 0.5s;

        &:hover,
        &:focus {
          background: #dadada;
        }

        button,
        input {
          -webkit-appearance: none;
          -moz-appearance: none;
          appearance: none;
          background: transparent;
          border: 0;
          color: inherit;
          font: inherit;
          outline: 0;
        }

        .search-input {
          flex: 1;
        }

        .search-button {
          cursor: pointer;
          color: #383838;
          display: flex;
          align-items: center;
          justify-content: center;
          &:hover {
            color: #000;
          }

          svg {
            width: 20px;
            height: 20px;
            font-size: 20px;
          }
        }
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
      height: 50px;
      display: flex;
      align-items: center;
      justify-content: space-evenly;

      button {
        background-color: #fff;
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 10px;
        cursor: pointer;
        transition: all 0.3s ease;
        overflow: hidden;
        outline: none;
        border: none;
        color: #000;

        &:hover {
          background-color: #dadada;
        }

        svg: {
          width: 20px;
          height: 20px;
        }
      }

      .button-group {
        width: 40%;
        display: flex;
        align-items: center;
        justify-content: space-around;
        height: 100%;

        .add-chat-btn {
          div {
            margin-left: 5px;
            font-size: 14px;
            color: #666;
          }
        }
      }
    }
  }

  .app-main {
    width: calc(100% - 283px);
    height: 100%;
  }
}
</style>
