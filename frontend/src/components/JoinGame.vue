<script lang="ts" setup>
import { type FormInst, type FormItemRule, type FormRules, useNotification } from 'naive-ui'
import { ref } from 'vue'

interface ModelType {
  roomId: string | null
  nickname: string | null
}
const formRef = ref<FormInst | null>(null)
const notification = useNotification()
const handleSubmit = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      console.log(model)
      notification.success({
        title: '成功',
        content: '加入成功',
      })
    } else {
      console.log(errors)
      notification.error({
        title: '失败',
        content: '验证失败',
      })
    }
  })
}
const model = ref<ModelType>({
  roomId: null,
  nickname: null,
})
const rules: FormRules = {
  roomId: [
    {
      required: true,
      validator(rule: FormItemRule, value: string) {
        if (!value) {
          return new Error('请输入房间ID')
        }
        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  nickname: [
    {
      required: true,
      validator(rule: FormItemRule, value: string) {
        if (!value) {
          return new Error('请输入昵称')
        }
        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
}
</script>

<template>
  <n-flex align="center" justify="center" vertical>
    <n-h2>加入游戏</n-h2>
    <n-form ref="formRef" :model="model" :rules="rules">
      <n-form-item label="房间ID" path="roomId">
        <n-input v-model:value="model.roomId" placeholder="请输入房间ID" @keydown.enter.prevent />
      </n-form-item>
      <n-form-item label="昵称" path="nickname">
        <n-input v-model:value="model.nickname" placeholder="请输入昵称" @keydown.enter.prevent />
      </n-form-item>
      <n-row :gutter="[0, 24]">
        <n-col :span="24">
          <div style="display: flex; justify-content: flex-end">
            <n-button round type="primary" @click="handleSubmit"> 加入 </n-button>
          </div>
        </n-col>
      </n-row>
    </n-form>
  </n-flex>
</template>

<style scoped></style>
