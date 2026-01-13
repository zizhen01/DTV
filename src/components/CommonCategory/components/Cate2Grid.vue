<template>
  <div class="relative flex flex-1 flex-col overflow-visible bg-transparent px-2 pb-1.5 pt-2">
    <div
      class="relative overflow-hidden pb-1.5"
      ref="cate2ContentRef"
    >
      <div class="pointer-events-none absolute inset-x-0 bottom-0 h-[18px] bg-[linear-gradient(to_bottom,color-mix(in_srgb,var(--bg-primary)_0%,transparent),color-mix(in_srgb,var(--bg-primary)_85%,transparent))] opacity-90"></div>
      <div
        class="max-h-full"
        :class="isExpandedInternal && hasMoreRowsInternal
          ? 'overflow-y-auto [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:h-0 [&::-webkit-scrollbar]:w-0'
          : 'overflow-hidden'"
      >
        <div class="grid grid-cols-[repeat(auto-fill,minmax(120px,1fr))] gap-2.5 justify-start pb-3" ref="cate2GridRef">
          <div
            v-for="cate2 in cate2List"
            :key="cate2.href"
            class="flex h-10 cursor-pointer items-center justify-center rounded-2xl border border-[var(--border)] bg-[var(--bg-tertiary)] px-3.5 text-[12.5px] font-bold text-[var(--text-secondary)]"
            :class="selectedCate2Href === cate2.href ? 'bg-[var(--bg-secondary)] text-[var(--text-primary)] font-semibold' : ''"
            @click="$emit('select', cate2)"
          >
            <div class="truncate text-center" :title="cate2.title">{{ cate2.title }}</div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="shouldShowExpandButtonInternal" class="relative z-[5] mt-0.5 inline-flex items-center gap-1.5 self-center rounded-full border border-[var(--border)] bg-[var(--bg-secondary)] px-4 py-1.5 text-[11.5px] font-semibold text-[var(--secondary-text)] shadow-[var(--shadow-low)] transition-all duration-200 hover:bg-[var(--bg-tertiary)] hover:text-[var(--text-primary)]" @click="handleToggleInternalExpand">
      <span>{{ isExpandedInternal ? '收起' : '展开' }}</span>
      <svg
        class="ml-0.5 h-3 w-3 transition-transform"
        :class="{ 'rotate-180': isExpandedInternal }"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
      >
        <path d="M6 9l6 6 6-6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed, nextTick } from 'vue'
import type { Category2 } from '../../../platforms/common/categoryTypes.ts'

const props = defineProps<{
  cate2List: Category2[]
  selectedCate2Href: string | null
  isExpanded: boolean
}>()

const emit = defineEmits<{
  (e: 'select', cate2: Category2): void
  (e: 'toggle-expand'): void
  (e: 'height-changed'): void
}>()

const CARD_ACTUAL_HEIGHT = 36;
const GRID_VERTICAL_GAP = 12;
const CONTENT_PADDING_BOTTOM = 6;
const GRID_INTERNAL_PADDING_BOTTOM = 12;

const TARGET_CONTENT_HEIGHT_FOR_ONE_ROW = CARD_ACTUAL_HEIGHT + GRID_INTERNAL_PADDING_BOTTOM + CONTENT_PADDING_BOTTOM;
const TARGET_CONTENT_HEIGHT_FOR_TWO_ROWS = (2 * CARD_ACTUAL_HEIGHT + GRID_VERTICAL_GAP) + GRID_INTERNAL_PADDING_BOTTOM + CONTENT_PADDING_BOTTOM - 14;
const EXPANDED_CONTENT_MAX_ROWS = 7;
const TARGET_CONTENT_HEIGHT_FOR_EXPANDED_MAX_ROWS = 
    (EXPANDED_CONTENT_MAX_ROWS * CARD_ACTUAL_HEIGHT + (EXPANDED_CONTENT_MAX_ROWS - 1) * GRID_VERTICAL_GAP) 
    + GRID_INTERNAL_PADDING_BOTTOM + CONTENT_PADDING_BOTTOM;

const cate2ContentRef = ref<HTMLElement | null>(null)
const cate2GridRef = ref<HTMLElement | null>(null)
const isExpandedInternal = ref(props.isExpanded)
const actualGridScrollHeight = ref(0)
const hasMoreRowsInternal = ref(false)

const refreshHeightNonAnimated = () => {
  if (cate2ContentRef.value) {
    cate2ContentRef.value.style.height = `${getCurrentTargetHeight(isExpandedInternal.value)}px`;
    nextTick(() => emit('height-changed'));
  }
};

const updateActualGridScrollHeightAndMoreRows = () => {
  nextTick(() => {
    if (cate2GridRef.value) {
      actualGridScrollHeight.value = cate2GridRef.value.scrollHeight;
    } else {
      actualGridScrollHeight.value = GRID_INTERNAL_PADDING_BOTTOM;
    }
    hasMoreRowsInternal.value = requiredHeightForAllGridItemsWithPadding.value > TARGET_CONTENT_HEIGHT_FOR_EXPANDED_MAX_ROWS;
    refreshHeightNonAnimated();
  });
};

watch(() => props.cate2List, () => {
  updateActualGridScrollHeightAndMoreRows();
  refreshHeightNonAnimated();
}, { deep: true });

watch(() => props.isExpanded, (newVal) => {
  if (isExpandedInternal.value !== newVal) {
    isExpandedInternal.value = newVal;
    refreshHeightNonAnimated();
  }
});

onMounted(() => {
  isExpandedInternal.value = props.isExpanded;
  updateActualGridScrollHeightAndMoreRows();
});

const requiredHeightForAllGridItemsWithPadding = computed(() => {
  return actualGridScrollHeight.value + CONTENT_PADDING_BOTTOM;
});

const shouldShowExpandButtonInternal = computed(() => {
  return requiredHeightForAllGridItemsWithPadding.value > TARGET_CONTENT_HEIGHT_FOR_TWO_ROWS;
});

const getCurrentTargetHeight = (expandedState: boolean) => {
  const naturalContentHeight = requiredHeightForAllGridItemsWithPadding.value;
  if (expandedState) {
    if (hasMoreRowsInternal.value) {
      return TARGET_CONTENT_HEIGHT_FOR_EXPANDED_MAX_ROWS;
    }
    return props.cate2List.length > 0 ? naturalContentHeight : GRID_INTERNAL_PADDING_BOTTOM + CONTENT_PADDING_BOTTOM; 
  } else {
    if (naturalContentHeight <= TARGET_CONTENT_HEIGHT_FOR_ONE_ROW) {
      return naturalContentHeight;
    }
    return TARGET_CONTENT_HEIGHT_FOR_TWO_ROWS;
  }
};

const handleToggleInternalExpand = () => {
  emit('toggle-expand'); 
};
</script>

