<script setup lang="ts"></script>

<template>
  <div class="flex-col items-center flex grow content-center w-full">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <label class="text-2xl"
      >Curve:
      <math xmlns="http://www.w3.org/1998/Math/MathML"
        ><msup><mi>y</mi><mn>2</mn></msup
        ><mo>=</mo><msup><mi>x</mi><mn>3</mn></msup
        ><mo>+</mo><mi><input class="inline w-4 rounded" type="text"></mi><mi>x</mi><mo>+</mo><mi>b</mi></math
      >
    </label>
  </div>
</template>
