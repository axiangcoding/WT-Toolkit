export interface CmdResult {
  code?: number;
  stdout?: string;
  stderr?: string;
}


export interface AppSettings {
  wt_root_path: string;
  wt_setting_path: string;
  wt_ext_cli_path: string;
  language: string;
}