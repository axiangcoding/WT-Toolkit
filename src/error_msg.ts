/*

pub enum RetCode {
    Success = 0,
    Error = 1,
    GetAppSettingsFailed = 10000,
    SaveAppSettingsFailed = 10001,
    GetAppLogPathFailed = 10002,
    AutoDetectedWtRootPathFailed = 20000,
    AutoDetectedWtSettingPathFailed = 20001,
    InstallUserSkinFailed = 20002,
    InstallUserSightFailed = 20003,
    GetUserSkinsFailed = 20004,
    GetUserSightsFailed = 20005,
}
*/

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
    case 10002:
      return "获取应用日志路径失败";
    case 20000:
      return "自动检测战争雷霆游戏根目录失败";
    case 20001:
      return "自动检测战争雷霆配置目录失败";
    case 20002:
      return "安装自定义涂装失败";
    case 20003:
      return "安装自定义瞄具失败";
    case 20004:
      return "获取自定义涂装失败";
    case 20005:
      return "获取自定义瞄具失败";

    default:
      return `未知错误: ${err_code}`;
  }
}
