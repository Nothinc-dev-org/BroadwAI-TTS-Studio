<script setup lang="ts">
type ButtonColor = 'primary' | 'secondary' | 'success' | 'info' | 'warning' | 'error' | 'neutral'

const props = withDefaults(defineProps<{
  open: boolean
  title?: string
  description?: string
  confirmLabel?: string
  cancelLabel?: string
  confirmColor?: ButtonColor
  confirmIcon?: string
}>(), {
  title: 'Confirmar acción',
  description: '¿Estás seguro de que quieres continuar?',
  confirmLabel: 'Confirmar',
  cancelLabel: 'Cancelar',
  confirmColor: 'error',
  confirmIcon: 'i-lucide-trash-2',
})

const emit = defineEmits<{
  'update:open': [value: boolean]
  confirm: []
  cancel: []
}>()

const isOpen = computed({
  get: () => props.open,
  set: val => emit('update:open', val),
})

function onConfirm() {
  isOpen.value = false
  emit('confirm')
}

function onCancel() {
  isOpen.value = false
  emit('cancel')
}
</script>

<template>
  <UModal v-model:open="isOpen" :title="title">
    <template #body>
      <p class="text-sm text-muted">
        {{ description }}
      </p>
      <div class="flex justify-end gap-2 pt-4">
        <UButton variant="ghost" @click="onCancel">
          {{ cancelLabel }}
        </UButton>
        <UButton
          :color="confirmColor"
          :icon="confirmIcon"
          @click="onConfirm"
        >
          {{ confirmLabel }}
        </UButton>
      </div>
    </template>
  </UModal>
</template>
