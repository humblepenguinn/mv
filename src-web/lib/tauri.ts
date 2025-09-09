import type { InvokeArgs } from '@tauri-apps/api/core';
import { invoke } from '@tauri-apps/api/core';

export type TauriCmd =
  | 'cmd_metadata'
  | 'cmd_check_for_updates'
  | 'cmd_download_and_install_update'
  | 'cmd_analyze_source_code'
  | 'cmd_get_system_fonts'
  | 'cmd_open_url';

export async function invokeCmd<T>(
  cmd: TauriCmd,
  args?: InvokeArgs
): Promise<T> {
  try {
    return await invoke(cmd, args);
  } catch (err) {
    console.warn('Tauri command error', cmd, err);
    throw err;
  }
}
