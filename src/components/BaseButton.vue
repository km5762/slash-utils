<template>
  <button
    :class="[
      !$attrs.disabled ? 'hover-overlay' : 'opacity-50 cursor-not-allowed',
      'border border-slate-700 rounded-md px-4 py-2 font-bold text-white',
    ]"
  >
    <slot></slot>
  </button>
</template>
