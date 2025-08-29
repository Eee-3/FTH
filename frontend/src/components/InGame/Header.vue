<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineEmits } from 'vue'

const props = defineProps<{
  id: string
  title: string
}>()

const emit = defineEmits(['exit-room'])

// 计时器相关
const startTime = ref<Date>(new Date())
const elapsedTime = ref<number>(0)
const timerInterval = ref<NodeJS.Timeout | null>(null)

// 格式化时间显示
const formattedTime = ref<string>('00:00:00')

// 更新计时器
const updateTimer = () => {
  const now = new Date()
  elapsedTime.value = Math.floor((now.getTime() - startTime.value.getTime()) / 1000)

  const hours = Math.floor(elapsedTime.value / 3600)
  const minutes = Math.floor((elapsedTime.value % 3600) / 60)
  const seconds = elapsedTime.value % 60

  formattedTime.value = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
}

// 处理退出房间
const handleExitRoom = () => {
  emit('exit-room')
}

onMounted(() => {
  // 启动计时器
  timerInterval.value = setInterval(updateTimer, 1000)
  updateTimer() // 立即更新一次
})

onUnmounted(() => {
  // 清理计时器
  if (timerInterval.value) {
    clearInterval(timerInterval.value)
  }
})
</script>

<template>
  <header class="header">
    <div class="header-left">
      <button class="exit-button" @click="handleExitRoom">退出房间</button>
    </div>

    <div class="header-center">
      <h1 class="title">{{ props.title }}</h1>
      <span class="room-id">ID: {{ props.id }}</span>
    </div>

    <div class="header-right">
      <span class="timer">{{ formattedTime }}</span>
    </div>
  </header>
</template>

<style scoped>
.header {
  //background-color: #4a5568;
  //color: white;
  padding: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  box-sizing: border-box;
}

.header-left {
  flex: 1;
  display: flex;
  justify-content: flex-start;
}

.header-center {
  flex: 2;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
}

.header-right {
  flex: 1;
  display: flex;
  justify-content: flex-end;
}

.exit-button {
  background-color: #e53e3e;
  color: white;
  border: none;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background-color 0.2s;
}

.exit-button:hover {
  background-color: #c53030;
}

.title {
  margin: 0;
  font-size: 1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.room-id {
  font-size: 0.8rem;
  opacity: 0.8;
  margin-top: 0.1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100%;
}

.timer {
  font-family: monospace;
  font-size: 1rem;
  font-weight: bold;
}
</style>
