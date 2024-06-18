export function get_error_msg(error: any): string {
  console.error("error", error);
  let err_code = error as number;
  switch (err_code) {
    case 1:
      return "发生了未知错误";
    case 10000:
      return "获取应用设置失败";
    case 10001:
      return "保存应用设置失败";
    default:
      return `未知错误: ${err_code}`;
  }
}
