import { path } from "@tauri-apps/api";
import { BaseDirectory, exists, readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { appDataDir } from "@tauri-apps/api/path";

import { invoke } from "@tauri-apps/api";

// 导出接口
export interface AppSettings {
    wt_install_path: string;
    wt_setting_path: string;
    wt_skins_backup_path: string;
}

export async function getDefaultSettings() {
    let appDataDirPath = await appDataDir();
    let skinBackupPath = await path.join(appDataDirPath, 'wt_skins_backup');

    const settings: AppSettings = {
        wt_install_path: '',
        wt_setting_path: '',
        wt_skins_backup_path: skinBackupPath,
    };

    return settings;
}


export async function getAppSettings() {
    let fileName = 'settings.json'
    let setting_str = ''
    if (await exists(fileName, { dir: BaseDirectory.AppConfig })) {
        setting_str = await readTextFile(fileName, { dir: BaseDirectory.AppConfig })
    } else {
        setting_str = JSON.stringify(await getDefaultSettings(), null, 2)
        await writeTextFile(fileName, setting_str, { dir: BaseDirectory.AppConfig })
    }
    let settings = JSON.parse(setting_str);
    await invoke('create_folder', { path: settings.wt_skins_backup_path })
    return settings;
}

export async function saveAppSettings(settings: AppSettings) {
    let fileName = 'settings.json'
    let setting_str = JSON.stringify(settings, null, 2)
    await writeTextFile(fileName, setting_str, { dir: BaseDirectory.AppConfig })
}