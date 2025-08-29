<script setup lang="ts">
import { RouterView } from 'vue-router'
import { lightTheme,darkTheme, type MenuOption, NH1, NIcon } from 'naive-ui'
import { type Component, h, ref } from 'vue'
import { Moon,Sunny } from '@vicons/ionicons5'
import { useDark, useToggle } from '@vueuse/core'

const theme = ref(darkTheme)
const isDark = useDark({
  onChanged(dark: boolean) {
    if (dark) {
      theme.value = darkTheme
    } else {
      theme.value = lightTheme
    }
  },
})
const toggleDark = useToggle(isDark)


</script>

<template>
  <n-config-provider :theme="theme">
    <n-notification-provider>
      <header>
      </header>
      <n-float-button :right="20" bottom="20" @click="toggleDark()">
        <n-icon>
          <Sunny v-if="!isDark"/>
          <Moon v-if="isDark"/>
        </n-icon>
      </n-float-button>
      <RouterView class="container"/>
    </n-notification-provider>
    <n-global-style />
  </n-config-provider>
</template>

<style scoped>
</style>
