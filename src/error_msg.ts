import i18n from "./i18n";

export function get_error_msg(error: any): string {
  console.error("error", error);
  let err_code = error as number;
  switch (err_code) {
    case 1:
      return i18n.global.t("error.1");
    case 10000:
      return i18n.global.t("error.10000");
    case 10001:
      return i18n.global.t("error.10001");
    case 10002:
      return i18n.global.t("error.10002");
    case 20000:
      return i18n.global.t("error.20000");
    case 20001:
      return i18n.global.t("error.20001");
    case 20002:
      return i18n.global.t("error.20002");
    case 20003:
      return i18n.global.t("error.20003");
    case 20004:
      return i18n.global.t("error.20004");
    case 20005:
      return i18n.global.t("error.20005");
    default:
      return i18n.global.t("error.unknown", { err_code: err_code });
  }
}

