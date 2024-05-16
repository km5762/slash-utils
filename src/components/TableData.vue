<script setup>
defineProps({ className: String });
</script>

<template>
  <td
    :class="'border-2 border-slate-900 text-center align-center ' + className"
  >
    <slot />
  </td>
</template>
