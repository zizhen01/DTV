<template>
  <div class="flex h-full flex-col overflow-hidden bg-transparent">
    <CommonCategory 
      :categories-data="biliCategoriesData as any"
      @category-selected="onCategorySelected"
      class="z-10 flex-shrink-0 bg-transparent [backdrop-filter:none]"
    />
    <CommonStreamerList 
      :selected-category="currentSelectedCategory"
      :categories-data="biliCategoriesData as any"
      platformName="bilibili"
      playerRouteName="bilibiliPlayer"
      class="flex-1 overflow-hidden bg-transparent"
    />
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'BilibiliHomeView'
})

import { ref } from 'vue'
import CommonCategory from '../components/CommonCategory/index.vue'
import CommonStreamerList from '../components/CommonStreamerList/index.vue'
import { biliCategoriesData } from '../platforms/bilibili/biliCategoriesData'
import type { CategorySelectedEvent } from '../platforms/common/categoryTypes.ts'

const currentSelectedCategory = ref<CategorySelectedEvent | null>(null)
const onCategorySelected = (categoryEvent: CategorySelectedEvent) => {
  currentSelectedCategory.value = categoryEvent
}
</script>

