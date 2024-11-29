<template>
  <div class="chat-room">
    <div class="chat-title">默认对话</div>
    <div class="chat-messages" id="scrollableContent">
      <div v-if="loadMore" class="load-more">加载更多...</div>
      <template v-for="i in messageCount" :key="i">
        <MessageBox v-if="i % 2 === 0" :value="aiMessage"></MessageBox>
        <MessageBox v-else :value="meMessage"></MessageBox>
      </template>
    </div>
    <div class="chat-sender"></div>
  </div>
</template>

<script lang="ts" setup>
import { nextTick, onMounted, reactive, ref } from "vue";
import MessageBox from "../components/MessageBox.vue";

const loadMore = ref<boolean>(false);
const content = document.getElementById("scrollableContent")!;
onMounted(() => {
  scrollToTopListener();
  nextTick(() => {
    scrollToBottom();
    loadMore.value = true;
  });
});

const scrollToBottom = () => {
  setTimeout(() => {
    content.scrollTop = content.scrollHeight;
  }, 300);
};

const isAtTop = ref<boolean>(false);
const scrollToTopListener = () => {
  content.addEventListener("scroll", () => {
    if (content.scrollTop === 0) {
      if (isAtTop.value) {
        // 滚动到顶部触发事件
        console.log("已经到达顶部，无法继续滚动。");
        messageCount.value += 4;
        isAtTop.value = false;
      } else {
        isAtTop.value = true;
      }
    }
  });
};

const messageCount = ref<number>(4);
const md = ref<string>(
  "# Tauri + Vue + TypeScript\nThis template should help get you starteemplate should help get you starteemplate should help get you starteemplate should help get you starteemplate should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more."
);
const aiMessage = reactive<Message>({
  id: "1",
  sender: "AI",
  content: md.value,
  createTime: "2024年11月30日 15:35:50",
});
const meMessage = reactive<Message>({
  id: "2",
  sender: "ME",
  content: "#12313",
  createTime: "2024年11月30日 15:35:50",
});
</script>

<style lang="scss" scoped>
.chat-room {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-around;

  .chat-title {
    height: 60px;
    line-height: 60px;
    font-size: 20px;
    font-weight: bolder;
    background-color: #fff;
    border-bottom: 1px solid #ccc;
    padding-left: 20px;
  }

  .chat-messages {
    flex: 1;
    padding-left: 10px;
    padding-right: 10px;
    overflow-y: auto;
    scroll-behavior: smooth;

    &::-webkit-scrollbar {
      width: 0;
    }
  }

  .chat-sender {
    height: 180px;
    border-top: 1px solid #ccc;
    background-color: #fff;
  }
}
</style>
