import en_us from "./locales/en_us.json";
import zh_cn from "./locales/zh_cn.json";

import { createI18n } from "vue-i18n";

const i18n = createI18n({
  legacy: false,
  locale: "en_us",
  fallbackLocale: "en_us",
  messages: {
    en_us,
    zh_cn,
  },
});

export default i18n;
