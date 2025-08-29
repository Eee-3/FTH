<script setup lang="ts">
import { useNotification } from 'naive-ui'
import CreateGame from '@/components/CreateGame.vue'
import JoinGame from '@/components/JoinGame.vue'

const notification = useNotification()
// const click_test = () => {
//   notification.success({
//     title: 'Success',
//     description: 'This is a success message',
//     duration: 2500,
//     keepAliveOnHover: true,
//   })
// }
</script>

<template>
  <main>
    <!--    <n-button type="primary" @click="click_test"> Primary </n-button>-->
    <n-flex justify="center" align="center" class="tabs-container">
      <n-card class="tabs">
        <n-tabs type="segment" animated>
          <n-tab-pane name="create" tab="加入对局">
            <JoinGame />
          </n-tab-pane>
          <n-tab-pane name="join" tab="创建对局">
            <CreateGame />
          </n-tab-pane>
        </n-tabs>
      </n-card>
      <n-card class="tabs" title="历史房间">
        <GameHistory />
      </n-card>
    </n-flex>
  </main>
</template>

<style scoped>
.tabs-container {
  height: 100vh;
}
.tabs {
  width: 25%;
}
</style>
