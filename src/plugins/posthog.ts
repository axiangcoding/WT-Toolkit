//./plugins/posthog.js
import posthog from "posthog-js";

export default {
  install(app: any) {
    app.config.globalProperties.$posthog = posthog.init(
      'phc_uoXwIHmTDhsk1jaSmWfElbyW3OWU89lzqXcQhv0GApc',
      {
        api_host: 'https://us.i.posthog.com',
      }
    );
  },
};