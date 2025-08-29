<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import Header from '@/components/InGame/Header.vue'
import { useRoute, useRouter } from 'vue-router'
import { useNotification } from 'naive-ui'

const route = useRoute()
const router = useRouter()

const notification = useNotification()
const outer = ref(0.25)
const op_panel_split = ref(0.85)
const header_split = ref(0.05)
watch(header_split,(new_val,_)=>header_split.value=Math.max(new_val,0.05))
const game_title = ref('测试标题')
const game_id = computed(() => {return route.params.id as string})

// 处理退出房间
const handleExitRoom = () => {
  notification.success({
    content: '成功退出房间',
    duration: 2000
  })
  // 导航回主页
  router.push('/')
}
</script>

<template>
  <main>
    <n-flex justify="center" align="center" vertical class="in-game-wrapper">
      <n-split v-model:size="header_split" direction="vertical">
        <template #1>
          <Header :title="game_title" :id="game_id" @exit-room="handleExitRoom" />
        </template>
        <template #2>
          <n-split v-model:size="outer" direction="horizontal">
            <template #1> 这里是用户列表 </template>
            <template #2>
              <n-split v-model:size="op_panel_split" direction="vertical">
                <template #1>
                  <n-split direction="vertical">
                    <template #1>
                      <n-split direction="horizontal">
                        <template #1> Pane 1 </template>
                        <template #2> Pane 2 </template>
                      </n-split></template
                    >
                    <template #2>
                      <n-split direction="horizontal">
                        <template #1> Pane 1 </template>
                        <template #2> Pane 2 </template>
                      </n-split>
                    </template>
                  </n-split>
                </template>
                <template #2> 这里是操作面板 </template>
              </n-split>
            </template>
          </n-split>
        </template>
      </n-split>
    </n-flex>
  </main>
</template>

<style scoped>
.in-game-wrapper {
  height: calc(100vh - 1rem);
  padding: 0.5rem;
}
</style>
