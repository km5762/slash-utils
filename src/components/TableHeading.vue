<script setup>
defineProps({ colspan: String, className: String });
</script>

<template>
  <th
    :class="'border-2 border-slate-900 font-black ' + className"
    :colspan="colspan"
  >
    <slot />
  </th>
</template>
