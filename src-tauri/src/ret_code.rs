use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Copy)]
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

impl Serialize for RetCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}
