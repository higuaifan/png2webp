<script setup lang="ts">
/**
 * @description
 * @author 阿怪
 * @date 2023/6/14 16:03
 * @version v1.0.0
 *
 * 江湖的业务千篇一律，复杂的代码好几百行。
 */
import { appWindow } from '@tauri-apps/api/window';

const toDrag = async () => {
  await appWindow.startDragging();
}

</script>

<template>

  <div class="drag-header m-cursor-pointer" @mousedown="toDrag">
    <m-divider/>
  </div>


</template>

<style scoped>

.drag-header {
  width: 100vw;
  height: 30px;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  justify-content: end;
}


</style>
