<template>
  <div class="flex h-full flex-col overflow-hidden bg-transparent">
    <CommonCategory 
      :categories-data="huyaCategoriesData as any"
      @category-selected="onCategorySelected"
      class="z-10 flex-shrink-0 bg-transparent [backdrop-filter:var(--glass-blur)]"
    />
    <CommonStreamerList 
      :selected-category="currentSelectedCategory"
      :categories-data="huyaCategoriesData as any"
      :default-page-size="120"
      playerRouteName="huyaPlayer"
      class="flex-1 overflow-hidden bg-transparent"
    />
  </div>
</template>

<script setup lang="ts">
defineOptions({
  name: 'HuyaHomeView'
})

import { ref } from 'vue'
import CommonCategory from '../components/CommonCategory/index.vue'
import { huyaCategoriesData } from '../platforms/huya/huyaCategoriesData'
import CommonStreamerList from '../components/CommonStreamerList/index.vue'
import type { CategorySelectedEvent } from '../platforms/common/categoryTypes.ts'

const currentSelectedCategory = ref<CategorySelectedEvent | null>(null)
const onCategorySelected = (categoryEvent: CategorySelectedEvent) => {
  currentSelectedCategory.value = categoryEvent
}
</script>

