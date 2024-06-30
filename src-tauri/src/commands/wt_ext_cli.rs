use log::{debug, error};

use crate::{ret_code::RetCode, WrappedState};

#[derive(Debug, serde::Serialize)]
pub struct CmdResult {
    code: i32,
    stdout: Option<String>,
    stderr: Option<String>,
}

#[tauri::command]
pub fn exec_wt_ext_cli(
    state: tauri::State<WrappedState>,
    args: Vec<String>,
) -> Result<CmdResult, RetCode> {
    let wt_ext_cli_path = get_wt_ext_cli_path(state)?;
    debug!("args: {:?}", args);
    let output = std::process::Command::new(wt_ext_cli_path)
        .args(args)
        .output()
        .map_err(|e| {
            error!("Failed to execute command: {}", e);
            RetCode::WTExtCliCommandFailed
        })?;
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);
    Ok(CmdResult {
        code: output.status.code().unwrap(),
        stdout: Some(stdout),
        stderr: Some(stderr),
    })
}

fn get_wt_ext_cli_path(state: tauri::State<WrappedState>) -> Result<String, RetCode> {
    let wt_ext_cli_path = state
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .config
        .clone()
        .wt_ext_cli_path
        .ok_or(RetCode::WTExtCliPathNotExist)?;
    debug!("wt_ext_cli_path: {}", wt_ext_cli_path);
    // check if the path is a directory, and wt_ext_cli.exe exists
    let wt_ext_cli_exec_path = std::path::Path::new(&wt_ext_cli_path).join("wt_ext_cli.exe");

    if !wt_ext_cli_exec_path.exists() {
        error!("wt_ext_cli.exe not found in the specified path");
        return Err(RetCode::WTExtCliPathNotExist);
    }
    debug!(
        "wt_ext_cli_exec_path: {}",
        wt_ext_cli_exec_path.to_string_lossy()
    );
    Ok(wt_ext_cli_exec_path.to_string_lossy().to_string())
}
