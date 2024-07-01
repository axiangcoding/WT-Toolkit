import en from "./locales/en.json";
import zh from "./locales/zh.json";

import { createI18n } from "vue-i18n";

const i18n = createI18n({
    legacy: false,
    locale: "en",
    fallbackLocale: "en",
    messages: {
      en,
      zh,
    },
  });

export default i18n;