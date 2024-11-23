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
    <div class="room-footer">
      <div class="sender-btns">
        <div class="btn-box clickable">
          <div class="btn-icon">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              width="16"
              height="16"
              fill="none"
            >
              <g>
                <mask id="chat-settings_svg__b" fill="#fff"><use xlink:href="#chat-settings_svg__a"></use></mask>
                <g mask="url(#chat-settings_svg__b)">
                  <path
                    d="M6.728 14.598a.665.665 0 0 1-.828.434 7.294 7.294 0 0 1-3.187-1.945.666.666 0 0 1-.026-.896c.207-.241.31-.527.31-.857 0-.37-.13-.685-.389-.944A1.286 1.286 0 0 0 1.663 10h-.08a.662.662 0 0 1-.665-.288.665.665 0 0 1-.098-.235 7.234 7.234 0 0 1 .188-3.675.662.662 0 0 1 .635-.465h.02c.37 0 .684-.13.944-.392.26-.26.39-.574.39-.942 0-.215-.045-.415-.134-.6a.666.666 0 0 1 .148-.78A7.292 7.292 0 0 1 6.034.932a.666.666 0 0 1 .774.34c.11.219.272.394.483.524.216.135.454.202.712.202.254 0 .488-.067.703-.201.211-.131.372-.306.483-.524a.666.666 0 0 1 .773-.34 7.404 7.404 0 0 1 3.03 1.688.664.664 0 0 1 .143.8c-.09.174-.135.368-.135.583 0 .366.13.68.392.941.262.262.575.393.941.393h.03a.664.664 0 0 1 .636.465c.22.695.331 1.43.331 2.201a7.34 7.34 0 0 1-.143 1.474.66.66 0 0 1-.27.411.666.666 0 0 1-.478.115 1 1 0 0 0-.106-.003c-.367 0-.681.13-.941.39-.261.26-.392.574-.392.943 0 .322.106.61.317.865a.662.662 0 0 1 .1.686.671.671 0 0 1-.133.202 7.294 7.294 0 0 1-3.187 1.945.674.674 0 0 1-.503-.047.659.659 0 0 1-.325-.387 1.311 1.311 0 0 0-.477-.667 1.288 1.288 0 0 0-.789-.26c-.297 0-.564.087-.798.26-.23.17-.389.393-.477.667zm3.52-1.032c.626-.253 1.191-.6 1.696-1.042a2.603 2.603 0 0 1-.277-1.19 2.642 2.642 0 0 1 .784-1.888 2.646 2.646 0 0 1 1.51-.753 6.393 6.393 0 0 0-.118-2.067 2.655 2.655 0 0 1-1.394-.738 2.656 2.656 0 0 1-.782-1.884c0-.239.029-.468.087-.687a6.05 6.05 0 0 0-1.693-.954c-.182.22-.398.41-.65.566-.431.268-.9.402-1.408.402-.511 0-.983-.134-1.414-.401a2.632 2.632 0 0 1-.653-.567 5.936 5.936 0 0 0-1.69.957 2.74 2.74 0 0 1-.125 1.72c-.134.319-.324.601-.57.847a2.641 2.641 0 0 1-1.389.738 5.903 5.903 0 0 0-.12 2.069 2.642 2.642 0 0 1 1.509.753 2.654 2.654 0 0 1 .779 1.888c0 .433-.091.831-.274 1.192.504.44 1.068.787 1.692 1.04.172-.272.393-.507.664-.707.47-.348 1-.522 1.591-.522.587 0 1.115.174 1.584.523.27.2.49.435.661.705z"
                    style="fill: rgb(51, 51, 51); opacity: 1"
                  ></path>
                  <path
                    d="M10.122 10.122a2.987 2.987 0 0 1-2.125.881 2.969 2.969 0 0 1-2.122-.883 2.983 2.983 0 0 1-.64-.955A2.984 2.984 0 0 1 5 7.997a2.974 2.974 0 0 1 .876-2.12A2.974 2.974 0 0 1 7.996 5c.407 0 .797.078 1.169.235.36.151.678.364.955.64a2.969 2.969 0 0 1 .883 2.122 2.987 2.987 0 0 1-.881 2.125zM7.997 6.333c-.461 0-.854.162-1.178.486a1.603 1.603 0 0 0-.486 1.178c0 .462.163.857.488 1.183.324.327.716.49 1.176.49.46 0 .855-.164 1.182-.491.327-.327.491-.721.491-1.182 0-.46-.163-.852-.49-1.176a1.618 1.618 0 0 0-1.183-.488z"
                    style="fill: rgb(51, 51, 51); opacity: 1"
                  ></path>
                </g>
              </g>
              <defs><path id="chat-settings_svg__a" d="M0 0h16v16H0z"></path></defs>
            </svg>
          </div>
          <div class="btn-text">对话设置</div>
        </div>
      </div>
      <div class="sender-box">
        <textarea
          v-model="sendMessage"
          class="sender-text-area"
          placeholder="Enter 发送，Shift + Enter 换行，/ 触发补全，: 触发命令"
        ></textarea>
        <div class="sender-text-btn" @click="handleSendMessage">
          <button>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              width="16"
              height="16"
              fill="none"
            >
              <defs><path id="send-white_svg__a" d="M0 0h16v16H0z"></path></defs>
              <g>
                <mask id="send-white_svg__b" fill="#fff"><use xlink:href="#send-white_svg__a"></use></mask>
                <g mask="url(#send-white_svg__b)">
                  <path
                    d="M0 4.71 6.67 6l1.67 6.67L12.67 0 0 4.71Z"
                    transform="translate(1.333 2)"
                    style="stroke: rgb(255, 255, 255); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                  <path
                    d="M0 1.89 1.89 0"
                    transform="translate(8.003 6.117)"
                    style="stroke: rgb(255, 255, 255); stroke-width: 1.33333; stroke-opacity: 1; stroke-dasharray: 0, 0"
                  ></path>
                </g>
              </g>
            </svg>
            <div>发送</div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref } from "vue";
import MessageBox from "./MessageBox.vue";

const props = defineProps<{ cb: ChatBox }>();

// watch(props, (newProps) => {
//   console.log(newProps.cb);
//   isFirstToTop.value = true;
// });

const hasScrollBar = ref<boolean>(false);
// other: 没有消息了
const loadMore = ref<string>("加载更多...");

// 每次分页查询后置位true
const isFirstToTop = ref<number>(0);
const loading = ref<boolean>(false);
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
      if (content.scrollTop === 0 && isFirstToTop.value > 0 && !loading.value) {
        loading.value = true;
        // 滚动到顶部触发事件，分页查询
        console.log("已经到达顶部，无法继续滚动。");
        // 查询完成后置为0
        isFirstToTop.value = 0;
        loading.value = false;
      }
      isFirstToTop.value += 1;
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

const sendMessage = ref<string>("");
const handleSendMessage = () => {
  const currentChat = props.cb;
  console.log(currentChat, sendMessage.value);
};
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
    padding: 0 10px;

    .sender-btns {
      height: 30px;
      display: flex;
      align-items: center;

      .btn-box {
        margin-top: 5px;
        display: inline-flex;
        border-radius: 20px;
        font-size: 12px;
        background-color: #fff;
        color: #303030;
        border: 1px solid #dedede;
        padding: 4px 10px;
        animation: chat_slide-in 0.3s ease;
        box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.05);
        transition: width 0.3s ease;
        align-items: center;
        height: 16px;
        width: 16px;
        overflow: hidden;

        .btn-icon {
          display: flex;
          align-items: center;
          justify-content: center;
        }

        .btn-text {
          white-space: nowrap;
          padding-left: 5px;
          opacity: 0;
          transform: translateX(-5px);
          transition: all 0.3s ease;
          pointer-events: none;
        }

        @keyframes chat_slide-in {
          0% {
            opacity: 0;
            transform: translateY(20px);
          }
          100% {
            opacity: 1;
            transform: translateY(0);
          }
        }
      }

      .clickable {
        cursor: pointer;

        &:hover {
          filter: brightness(0.9);
          width: 69px;
          transition-delay: 0.5s;

          .btn-text {
            transition-delay: 0.5s;
            opacity: 1;
            transform: translate(0);
          }
        }
      }
    }

    .sender-box {
      margin-top: 10px;
      height: 100px;
      border: 1px solid #ccc;
      border-radius: 10px;
      cursor: text;
      position: relative;

      &:has(.sender-text-area:focus) {
        border: 1px solid #1d93ab;
      }

      .sender-text-area {
        height: 100%;
        width: 100%;
        border-radius: 10px;
        border: none;
        box-shadow: 0 -2px 5px rgba(0, 0, 0, 0.03);
        background-color: #fff;
        color: #303030;
        font-family: inherit;
        padding: 10px 90px 10px 14px;
        resize: none;
        outline: none;
        box-sizing: border-box;
        min-height: 68px;
        font-size: 14px;
      }

      .sender-text-btn {
        position: absolute;
        bottom: 10px;
        right: 10px;
        display: flex;
        align-items: center;
        justify-content: space-around;

        button {
          background-color: #1d93ab;
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

          &:hover {
            filter: brightness(0.9);
          }

          svg: {
            width: 20px;
            height: 20px;
            color: #fff;
            fill: none;
          }
        }

        div {
          margin-left: 5px;
          font-size: 14px;
          color: #fff;
        }
      }
    }
  }
}
</style>
