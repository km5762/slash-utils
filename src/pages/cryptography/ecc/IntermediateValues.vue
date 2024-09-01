<script setup lang="ts">
import Container from "@/components/Container.vue";
import { useIntermediateValuesStore } from "./IntermediateValuesStore";
import { watch } from "vue";
import { pinia } from "@/pinia";
import { storeToRefs } from "pinia";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
const { selectedMode, signingIntermediateValues, verifyingIntermediateValues } =
  storeToRefs(intermediateValuesStore);
</script>

<template>
  <template v-if="selectedMode === 'Sign'">
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
          = {{ signingIntermediateValues?.hash }}
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
          = {{ signingIntermediateValues?.truncated_hash }}
        </Container>
      </div>
      <div>
        <math xmlns="http://www.w3.org/1998/Math/MathML"
          ><mo>(</mo><mi>x</mi><mo>,</mo><mi>y</mi><mo>)</mo><mo>=</mo><mi>k</mi
          ><mo>&#xd7;</mo><mi>G</mi> <mo>mod</mo> <mi>p</mi></math
        >
        <Container class="mt-2 break-all box-content w-[calc(32ch)]">
          = ({{ signingIntermediateValues?.generated_point.x }},
          {{ signingIntermediateValues?.generated_point.y }})
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
          = {{ signingIntermediateValues?.signature.r }}
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
          = {{ signingIntermediateValues?.signature.s }}
        </Container>
      </div>
    </div>
  </template>
  <template v-else>
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
          = {{ verifyingIntermediateValues?.hash }}
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
          = {{ verifyingIntermediateValues?.truncated_hash }}
        </Container>
      </div>
      <div>
        <math xmlns="http://www.w3.org/1998/Math/MathML">
          <msub>
            <mi>u</mi>
            <mn>1</mn>
          </msub>
          <mo>=</mo>
          <mi>z</mi>
          <mo>&#x2219;</mo>
          <msup>
            <mi>s</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
          </msup>
          <mo>mod</mo>
          <mi>n</mi></math
        >
        <Container class="mt-2 break-all box-content w-[calc(32ch)]">
          = {{ verifyingIntermediateValues?.u1 }}
        </Container>
      </div>
      <div>
        <math xmlns="http://www.w3.org/1998/Math/MathML">
          <msub>
            <mi>u</mi>
            <mn>2</mn>
          </msub>
          <mo>=</mo>
          <mi>r</mi>
          <mo>&#x2219;</mo>
          <msup>
            <mi>s</mi>
            <mrow><mo>-</mo><mn>1</mn></mrow>
          </msup>
          <mo>mod</mo>
          <mi>n</mi></math
        >
        <Container class="mt-2 break-all box-content w-[calc(32ch)]">
          = {{ verifyingIntermediateValues?.u1 }}
        </Container>
      </div>
      <div></div>
      <div>
        <math xmlns="http://www.w3.org/1998/Math/MathML"
          ><mo>(</mo><msub><mi>x</mi><mn>1</mn></msub
          ><mo>,</mo><msub><mi>y</mi><mn>1</mn></msub
          ><mo>)</mo><mo>=</mo><msub><mi>u</mi><mn>1</mn></msub
          ><mo>&#xd7;</mo><mi>G</mi><mo>+</mo><msub><mi>u</mi><mn>2</mn></msub
          ><mo>&#xd7;</mo><msub><mi>Q</mi><mi>A</mi></msub></math
        >
        <Container class="mt-2 break-all box-content w-[calc(32ch)]">
          = ({{ verifyingIntermediateValues?.generated_point.x }},
          {{ verifyingIntermediateValues?.generated_point.y }})
        </Container>
      </div>
    </div>
  </template>
</template>
