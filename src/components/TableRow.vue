<script setup>
defineProps({ className: String });
</script>

<template>
  <tr :class="'border border-slate-700 rounded-xl ' + className">
    <slot />
  </tr>
</template>
