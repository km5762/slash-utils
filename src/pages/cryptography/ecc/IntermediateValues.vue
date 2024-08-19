<script setup lang="ts">
import Container from "@/components/Container.vue";
import { useIntermediateValuesStore } from "./IntermediateValuesStore";
import { watch } from "vue";
import { pinia } from "@/pinia";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
</script>

<template>
  <h3 class="font-bold underline text-3xl pb-4 text-center">
    Intermediate Values
  </h3>
  <div class="flex flex-col gap-4">
    <div>
      <math xmlns="http://www.w3.org/1998/Math/MathML"
        ><mi>e</mi><mo>=</mo><mi>H</mi><mi>A</mi><mi>S</mi><mi>H</mi><mo>(</mo
        ><mi>m</mi><mo>)</mo></math
      >
      <Container class="mt-2 break-all box-content w-[32ch] h-[4ch]">
        = {{ intermediateValuesStore.e }}
      </Container>
    </div>
    <div>
      <math xmlns="http://www.w3.org/1998/Math/MathML"
        ><mi>z</mi><mo>=</mo><mi>T</mi><mi>R</mi><mi>U</mi><mi>N</mi><mi>C</mi
        ><mi>A</mi><mi>T</mi><mi>E</mi><mo>(</mo><mi>e</mi><mo>,</mo><mi>B</mi
        ><mi>I</mi><mi>T</mi><mi>L</mi><mi>E</mi><mi>N</mi><mo>(</mo><mi>n</mi
        ><mo>)</mo><mo>)</mo></math
      >
      <Container class="mt-2 break-all box-content w-[calc(32ch)]">
        = {{ intermediateValuesStore.z }}
      </Container>
    </div>
    <div>
      <math xmlns="http://www.w3.org/1998/Math/MathML"
        ><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo><mo>=</mo><mi>k</mi
        ><mo>&#xd7;</mo><mi>G</mi> <mo>mod</mo> <mi>p</mi></math
      >
      <Container class="mt-2 break-all box-content w-[calc(32ch)]">
        = {{ intermediateValuesStore.x }}
      </Container>
    </div>
    <div>
      <math xmlns="http://www.w3.org/1998/Math/MathML">
        <mrow>
          <mi>r</mi>
          <mo>=</mo>
          <mi>x</mi>
          <mo>mod</mo>
          <mi>n</mi>
        </mrow>
      </math>
      <Container class="mt-2 break-all box-content w-[calc(32ch)]">
        = {{ intermediateValuesStore.r }}
      </Container>
    </div>
    <div></div>
    <div>
      <math xmlns="http://www.w3.org/1998/Math/MathML">
        <mrow>
          <mi>s</mi>
          <mo>=</mo>
          <msup>
            <mi>k</mi>
            <mo>-1</mo>
          </msup>
          <mo>(</mo>
          <mi>z</mi>
          <mo>+</mo>
          <mi>r</mi>
          <mo>&#x22C5;</mo>
          <msub>
            <mi>d</mi>
            <mi>a</mi>
          </msub>
          <mo>)</mo>
          <mo>mod</mo>
          <mi>n</mi>
        </mrow>
      </math>
      <Container class="mt-2 break-all box-content w-[calc(32ch)]">
        = {{ intermediateValuesStore.s }}
      </Container>
    </div>
  </div>
</template>
