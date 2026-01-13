<template>
  <div class="flex h-full flex-col bg-transparent">
    <div class="flex h-full flex-col">
      <div class="z-10 w-full overflow-hidden bg-transparent [backdrop-filter:none]">
        <CommonCategory 
          :categoriesData="categoriesData"
          @category-selected="onCategorySelected" 
        />
      </div>
      <div class="flex-1 overflow-hidden bg-transparent">
        <CommonStreamerList
          :selectedCategory="selectedCategory"
          :categoriesData="categoriesData"
          platformName="douyin"
          playerRouteName="douyinPlayer"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import CommonCategory from '../components/CommonCategory/index.vue'
import CommonStreamerList from '../components/CommonStreamerList/index.vue'
import { douyinCategoriesData } from '../platforms/douyin/douyinCategoriesData'
import type { CategorySelectedEvent } from '../platforms/common/categoryTypes'

defineOptions({
  name: 'DouyinHomeView',
})

const categoriesData = douyinCategoriesData
const selectedCategory = ref<CategorySelectedEvent | null>(null)

function onCategorySelected(evt: CategorySelectedEvent) {
  selectedCategory.value = evt
}
</script>

