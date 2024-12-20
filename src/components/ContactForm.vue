<script setup lang="ts">
import TextInput from "./TextInput.vue";
import TextArea from "./TextArea.vue";
import Label from "./Label.vue";
import Container from "./Container.vue";
import BaseButton from "./BaseButton.vue";
</script>

<template>
  <form action="">
    <Container class="flex flex-col gap-4"
      ><h2 class="text-2xl font-bold mb-4">Send Us a Message</h2>
      <div class="flex gap-6 flex-grow">
        <div class="flex-grow">
          <Label for="first-name">First name</Label>
          <TextInput id="first-name" class="w-full h-8" />
        </div>
        <div class="flex-grow">
          <Label for="last-name">Last name</Label>
          <TextInput id="last-name" class="w-full h-8" />
        </div>
      </div>
      <div class="flex-grow">
        <Label for="last-name">Email</Label>
        <TextInput id="last-name" class="h-8 w-full" />
      </div>
      <div class="flex-grow">
        <Label for="last-name">Message</Label>
        <TextArea id="last-name" class="min-h-32 w-full" />
      </div>
      <BaseButton class="bg-teal-600">Send Message</BaseButton>
    </Container>
  </form>
</template>
