<script setup lang="ts"></script>

<template>
  <main>
    <n-flex justify="center" align="center" vertical class="in-game-wrapper">
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
    </n-flex>
  </main>
</template>

<style scoped>
.in-game-wrapper {
  height: calc(100vh - 1rem);
  padding: 0.5rem;
}
</style>
