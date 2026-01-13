<template>
  <div v-if="!isLoading && (cate3List.length > 0 || hasAllOption)" class="mx-2 mt-1.5 flex flex-wrap gap-2 pb-2">
    <div
      class="inline-flex h-7 cursor-pointer items-center rounded-full border border-[var(--border)] bg-[var(--bg-tertiary)] px-3 text-[11.5px] font-bold text-[var(--text-secondary)] shadow-[var(--shadow-low)] transition-all duration-200 ease-[cubic-bezier(0.16,1,0.3,1)] hover:-translate-y-0.5 hover:text-[var(--text-primary)]"
      :class="selectedCate3Id === null || selectedCate3Id === 'all' ? 'bg-[var(--bg-secondary)] text-[var(--text-primary)] font-semibold shadow-[var(--shadow-md)]' : ''"
      @click="selectAll"
    >
      全部
    </div>
    <div
      v-for="cate3 in cate3List"
      :key="cate3.id"
      class="inline-flex h-7 cursor-pointer items-center rounded-full border border-[var(--border)] bg-[var(--bg-tertiary)] px-3 text-[11.5px] font-bold text-[var(--text-secondary)] shadow-[var(--shadow-low)] transition-all duration-200 ease-[cubic-bezier(0.16,1,0.3,1)] hover:-translate-y-0.5 hover:text-[var(--text-primary)]"
      :class="selectedCate3Id === cate3.id ? 'bg-[var(--bg-secondary)] text-[var(--text-primary)] font-semibold shadow-[var(--shadow-md)]' : ''"
      @click="$emit('select', cate3)"
    >
      {{ cate3.name }}
    </div>
  </div>
  <div v-if="isLoading" class="flex items-center justify-center p-2.5 text-[13px] text-[var(--cate3-loading-text-dark,rgba(255,255,255,0.5))]">正在加载三级分类...</div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type Category3 = {
  id: string
  name: string
}

const props = defineProps<{
  cate3List: Category3[]
  selectedCate3Id: string | null
  isLoading: boolean
}>()

const emit = defineEmits<{
  (e: 'select', cate3: Category3): void
}>()

const hasAllOption = computed(() => {
  return props.cate3List && props.cate3List.length > 0
})

const selectAll = () => {
  const allCategory: Category3 = {
    id: 'all',
    name: '全部'
  }
  emit('select', allCategory)
}
</script>

