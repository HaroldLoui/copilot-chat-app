<template>
  <div class="chat-room">
    <div class="chat-title">默认对话</div>
    <div class="chat-messages" id="scrollableContent">
      <div v-if="hasNextPage" class="load-more">加载更多...</div>
      <template v-for="i in messageCount" :key="i">
        <MessageBox v-if="i % 2 === 0" :value="aiMessage"></MessageBox>
        <MessageBox v-else :value="meMessage"></MessageBox>
      </template>
    </div>
    <div class="chat-sender">
      <div class="sender-toolbar">
        <vs-button icon color="#909399" type="border">
          <i class="bx bx-smile"></i>
        </vs-button>
        <vs-button icon color="#909399" type="border">
          <i class="bx bxs-image"></i>
        </vs-button>
        <vs-button icon color="#909399" type="border">
          <i class="bx bxs-invader"></i>
        </vs-button>
      </div>
      <div class="sender-box">
        <textarea class="content-input">{{ senderInfo }}</textarea>
        <div class="sender-btn">
          <vs-button type="relief">
            <i class="bx bxs-paper-plane"></i>
            发送
          </vs-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { nextTick, onMounted, reactive, ref } from "vue";
import MessageBox from "../components/MessageBox.vue";

const hasNextPage = ref<boolean>(false);
onMounted(() => {
  const content = document.getElementById("scrollableContent")!;
  scrollToTopListener(content);
  nextTick(() => {
    scrollToBottom(content);
    hasNextPage.value = true;
  });
});

const scrollToBottom = (content: HTMLElement) => {
  setTimeout(() => {
    content.scrollTop = content.scrollHeight;
  }, 200);
};

const isAtTop = ref<boolean>(false);
const scrollToTopListener = (content: HTMLElement) => {
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

const senderInfo = ref<string>("213");
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

    .load-more {
      text-align: center;
      font-size: 12px;
      color: rgb(var(--vs-success-light-3));
      cursor: pointer;

      &:hover {
        color: rgb(var(--vs-success-light-7));
      }
    }
  }

  .chat-sender {
    height: 180px;
    border-top: 1px solid #ccc;
    background-color: #fff;

    .sender-toolbar {
      height: 50px;
      display: flex;
      align-items: center;
      justify-content: flex-start;
    }

    .sender-box {
      position: relative;
      padding: 10px;
      padding-top: 0;

      .content-input {
        height: 110px;
        display: inline-block;
        padding: 10px 90px 10px 10px;
        width: 100%;
        border-color: #ccc;
        border-radius: 5px;
        resize: none;
        &::-webkit-scrollbar {
          width: 0;
        }
        &:focus {
          border-color: rgba(var(--vs-primary-light-5));
        }
      }

      .sender-btn {
        position: absolute;
        bottom: 25px;
        right: 10px;
        width: 90px;
        height: 40px;
      }
    }
  }
}
</style>
