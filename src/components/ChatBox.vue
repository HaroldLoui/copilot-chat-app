<template>
  <div class="chat-box">
    <div class="close-btn" @click="handleClose">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
        <path
          fill="currentColor"
          d="m466.752 512-90.496-90.496a32 32 0 0 1 45.248-45.248L512 466.752l90.496-90.496a32 32 0 1 1 45.248 45.248L557.248 512l90.496 90.496a32 32 0 1 1-45.248 45.248L512 557.248l-90.496 90.496a32 32 0 0 1-45.248-45.248z"
        ></path>
        <path
          fill="currentColor"
          d="M512 896a384 384 0 1 0 0-768 384 384 0 0 0 0 768m0 64a448 448 0 1 1 0-896 448 448 0 0 1 0 896"
        ></path>
      </svg>
    </div>
    <div class="box-content">
      <div class="box-content-row row-title">
        {{ value.title }}
      </div>
      <div class="box-content-row row-sub-title">
        <div class="count">{{ value.count }}条对话</div>
        <div class="time">{{ value.createTime }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{ value: ChatBox }>();
const emits = defineEmits(["close"]);

const handleClose = () => {
  const { id } = props.value;
  if (id) {
    console.log("删除当前对话", id);
    emits("close", true);
  } else {
    emits("close", false);
  }
};
</script>

<style lang="scss" scoped>
.chat-box {
  margin-bottom: 10px;
  height: 60px;
  border: 2px solid #2a94a1;
  border-radius: 5px;
  background-color: #fff;
  transition: background-color 0.3s ease;
  position: relative;

  &:hover {
    cursor: pointer;
    background-color: #ebedee;
  }

  &:hover .close-btn {
    opacity: 1;
  }

  .close-btn {
    width: 20px;
    height: 20px;
    color: #949494;
    transition: color 0.3s ease;
    position: absolute;
    top: 3px;
    right: 5px;
    opacity: 0;
    transition: opacity 0.3s ease;

    &:hover {
      color: #5f5f5f;
    }
  }

  .box-content {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    .box-content-row {
      height: 36%;
    }
    .row-title {
      font-size: 14px;
      font-weight: bolder;
      padding-left: 5px;
      color: #303030;
    }
    .row-sub-title {
      font-size: 12px;
      display: flex;
      justify-content: space-around;
      align-items: end;
      color: #a6a6a6;
      .count {
        max-width: 25%;
        padding-left: 5px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
      .time {
        width: 75%;
        text-align: right;
        padding-right: 5px;
      }
    }
  }
}
</style>
