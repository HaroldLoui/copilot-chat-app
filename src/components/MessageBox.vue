<template>
  <div class="message-box">
    <div class="message-header" :class="{ reverse: value.sender === 'ME' }">
      <div class="avatar">{{ value.sender }}</div>
      <div class="send-time">{{ value.createTime }}</div>
    </div>
    <div class="flex-content" :class="{ 'flex-end': value.sender === 'ME' }">
      <div class="message-content">
        <MdPreview class="md-preview" :editorId="'preview-only-' + value.id" :modelValue="value.content" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MdPreview } from "md-editor-v3";
import "md-editor-v3/lib/preview.css";

defineProps<{ value: Message }>();
</script>

<style scoped>
.message-box {
  margin-top: 25px;

  .message-header {
    display: flex;
    flex-direction: row;
    align-items: flex-end;
    justify-content: flex-start;
    gap: 15px;

    .avatar {
      width: 32px;
      height: 32px;
      border: 1px solid #ccc;
      border-radius: 5px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #686868;
      font-weight: bolder;
    }

    .send-time {
      height: 15px;
      font-size: 12px;
      color: #a7a7a7;
    }
  }

  .reverse {
    flex-direction: row-reverse !important;
  }

  .flex-content {
    display: flex;

    .message-content {
      margin-top: 10px;
      display: inline-block;
      min-width: 35px;
      min-height: 35px;
      max-width: 100%;
      border: 1px solid #ccc;
      border-radius: 10px;
      background-color: rgba(0, 0, 0, 0.05);
      box-sizing: border-box;
      justify-content: flex-end;

      .md-preview {
        border-radius: 10px;
      }
    }
  }

  .flex-end {
    justify-content: flex-end;
  }
}
</style>

<style>
.md-preview > .md-editor-preview-wrapper {
  padding: 0 10px;
}
</style>
