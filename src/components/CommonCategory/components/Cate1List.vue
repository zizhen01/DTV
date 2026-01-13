<template>
  <div class="flex flex-shrink-0 overflow-x-auto border-none p-1 pb-2.5 shadow-none transition-all duration-200 ease-in-out [-ms-overflow-style:none] [scrollbar-width:none] [&::-webkit-scrollbar]:h-0">
    <ul class="relative inline-flex flex-nowrap gap-[18px] bg-transparent px-2 pb-1.5">
      <li
        v-for="cate1 in cate1List"
        :key="cate1.href"
        class="relative inline-flex h-8 cursor-pointer items-center rounded-md border border-transparent bg-transparent px-1 text-[12.5px] font-medium text-[var(--secondary-text)] transition-[color,transform] duration-200 ease-in-out hover:text-[var(--primary-text)]"
        :class="cate1.href === selectedCate1Href ? 'font-bold text-[var(--text-primary)] -translate-y-0.5' : ''"
        @click="$emit('select', cate1)"
      >
        <span class="tracking-[0.2px]">{{ cate1.title }}</span>
        <span
          class="absolute -bottom-1.5 left-[22%] right-[22%] h-1 rounded-full bg-[color-mix(in_srgb,var(--text-primary)_65%,transparent)] transition-[opacity,transform] duration-200 ease-[cubic-bezier(0.16,1,0.3,1)]"
          :class="cate1.href === selectedCate1Href ? 'opacity-100 scale-x-100' : 'opacity-0 scale-x-75'"
        ></span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import type { Category1 } from '../../../platforms/common/categoryTypes.ts'

defineProps<{
  cate1List: Category1[]
  selectedCate1Href: string | null
}>()
</script>

