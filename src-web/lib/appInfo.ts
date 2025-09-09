import { invokeCmd } from './tauri';

const isDesktop =
  typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;

export interface AppInfo {
  isDev: boolean;
  isDesktop: boolean;
  os?: string;
  version: string;
  name: string;
  appDataDir?: string;
  appLogDir?: string;
}

declare const __APP_VERSION__: string;
declare const __APP_NAME__: string;

const getMetadata = async (): Promise<Omit<AppInfo, 'isDesktop'>> => {
  if (isDesktop) {
    return await invokeCmd('cmd_metadata');
  } else {
    return {
      isDev: import.meta.env.DEV,
      os: 'web',
      version: __APP_VERSION__,
      name: __APP_NAME__,
    };
  }
};

export const appInfo = {
  ...(await getMetadata()),
  isDesktop,
} as AppInfo;
