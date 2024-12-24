<script setup lang="ts">
import TreeView from "@/components/TreeView.vue";
import { ref } from "vue";

const items = [
  {
    id: 0,
    title: "composables",
    icon: "lucide:folder",
    children: [
      { id: 1, title: "useAuth.ts", icon: "vscode-icons:file-type-typescript" },
      { id: 2, title: "useUser.ts", icon: "vscode-icons:file-type-typescript" },
    ],
  },
  {
    id: 3,
    title: "components",
    icon: "lucide:folder",
    children: [
      {
        id: 4,
        title: "Home",
        icon: "lucide:folder",
        children: [
          { id: 5, title: "Card.vue", icon: "vscode-icons:file-type-vue" },
          { id: 6, title: "Button.vue", icon: "vscode-icons:file-type-vue" },
        ],
      },
    ],
  },
  { id: 7, title: "app.vue", icon: "vscode-icons:file-type-vue" },
  { id: 8, title: "nuxt.config.ts", icon: "vscode-icons:file-type-nuxt" },
];

const selected = ref(null);
</script>

<template>
  <TreeView :nodes="items" :selected="selected" v-model="selected"></TreeView>
</template>
