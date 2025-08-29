<script setup lang="ts">
import { useNotification, type FormInst, type FormItemRule, type FormRules } from 'naive-ui'
import { ref } from 'vue'

interface ModelType {
  title: string | null
  nickname: string | null
  gameTemplate: number | null
}
const formRef = ref<FormInst | null>(null)
// const rPasswordFormItemRef = ref<FormItemInst | null>(null)
// const message = useMessage()
const notification = useNotification()
const handleSubmit = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      console.log(model)
      notification.success({
        title: '成功',
        content: '创建成功',
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
  title: null,
  nickname: null,
  gameTemplate: null,
})
const options = [
  {
    label: '模版0',
    value: 0,
  },
]
const rules: FormRules = {
  title: [
    {
      required: true,
      validator(rule: FormItemRule, value: string) {
        if (!value) {
          return new Error('请输入游戏标题')
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
  gameTemplate: [
    {
      required: true,
      message: '请选择游戏模版',
      // trigger: ['input', 'blur'],
    },
  ],
}
</script>

<template>
  <n-flex justify="center" align="center" vertical>
    <n-h2>创建游戏</n-h2>
    <n-form ref="formRef" :model="model" :rules="rules">
      <n-form-item path="title" label="游戏标题">
        <n-input v-model:value="model.title" placeholder="请输入游戏标题" @keydown.enter.prevent />
      </n-form-item>
      <n-form-item path="nickname" label="昵称">
        <n-input v-model:value="model.nickname" placeholder="请输入昵称" @keydown.enter.prevent />
      </n-form-item>
      <n-form-item path="gameTemplate" label="游戏模版">
        <n-select v-model:value="model.gameTemplate"  placeholder="请选择游戏模版" :options="options" />
      </n-form-item>
      <n-row :gutter="[0, 24]">
        <n-col :span="24">
          <div style="display: flex; justify-content: flex-end">
            <n-button round type="primary" @click="handleSubmit"> 创建 </n-button>
          </div>
        </n-col>
      </n-row>
    </n-form>
  </n-flex>
</template>

<style scoped></style>
