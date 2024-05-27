import type { App } from "vue";
import { createPinia } from "pinia";

export default (app: App) => {
  const pinia = createPinia();
  app.use(pinia);

  app.directive("numeric", (el) => {
    el.addEventListener("input", function (e) {
      this.value = this.value.replace(/\D/g, "");
    });
  });
};
