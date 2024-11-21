<template>
  <div class="room-container">
    <div class="room-header">
      <div class="chat-text">
        <div class="chat-title">{{ cb.title }}</div>
        <div class="chat-count">共 {{ cb.count }} 条对话</div>
      </div>
      <div class="chat-btns"></div>
    </div>
    <div id="scrollableContent" class="room-content">
      <div v-if="hasScrollBar" class="more-message">{{ loadMore }}</div>
      <template v-for="message in messageList" :key="message.id">
        <message-box :value="message"></message-box>
      </template>
    </div>
    <div class="room-footer"></div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import MessageBox from "./MessageBox.vue";

defineProps<{ cb: ChatBox }>();

const hasScrollBar = ref<boolean>(false);
// other: 没有消息了
const loadMore = ref<string>("加载更多...");

onMounted(() => {
  // 滑动到消息列表的底部
  document.addEventListener("DOMContentLoaded", () => {
    var content = document.getElementById("scrollableContent")!;
    // 计算是否出现滚动条
    hasScrollBar.value = content.scrollHeight > content.clientHeight;
    // 滚动到最底部的代码
    nextTick(() => {
      content.scrollTop = content.scrollHeight;
    });

    content.addEventListener("scroll", () => {
      if (content.scrollTop === 0) {
        // 滚动到顶部触发事件
        // message.textContent = "已经到达顶部，无法继续滚动。";
        console.log("已经到达顶部，无法继续滚动。");
      }
    });
  });
});

const messageList = ref<Message[]>([
  {
    id: "1",
    sender: "AI",
    content:
      "# Tauri + Vue + TypeScript\nThis template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.\n## Recommended IDE Setup\n- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)\n## Type Support For `.vue` Imports in TS\nSince TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:\n1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.\n2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.\nYou can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).\n",
    createTime: "2024年11月21日 14:57:32",
  },
  {
    id: "2",
    sender: "ME",
    content: "1231323132",
    createTime: "2024年11月21日 14:57:32",
  },
]);
</script>

<style lang="scss" scoped>
.room-container {
  display: flex;
  height: 100%;
  flex-direction: column;

  .room-header {
    height: 70px;
    border-bottom: 1px solid #ccc;
    background-color: #fff;
    display: flex;
    align-items: center;
    padding-left: 20px;

    .chat-text {
      width: 50%;

      .chat-title {
        height: 50%;
        font-size: 20px;
        font-weight: bolder;
      }

      .chat-count {
        height: 50%;
        line-height: 25px;
        font-size: 12px;
        color: #686868;
      }
    }

    .chat-btns {
      width: 50%;
    }
  }

  .room-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: #fff;
    padding: 10px;
    padding-top: 0;
    scroll-behavior: smooth;

    &::-webkit-scrollbar {
      width: 0;
    }

    .more-message {
      text-align: center;
      height: 20px;
      line-height: 20px;
      font-size: 12px;
      color: #88c2e9;
      cursor: pointer;
    }
  }

  .room-footer {
    border-top: 1px solid #ccc;
    height: 150px;
    background-color: azure;
  }
}
</style>
