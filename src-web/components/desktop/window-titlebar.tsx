import { appInfo } from '@/lib/appInfo';
import { WindowControls } from './window-controls';
import { SettingsButton } from '../ui/settings-button';

export function WindowTitleBar() {
  const isMac = appInfo.os === 'macos';

  const baseClasses =
    'bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 border-b border-border';

  if (isMac) {
    return (
      <div
        className={`h-[34px] flex items-center justify-end ${baseClasses}`}
        data-tauri-drag-region
      >
        <SettingsButton />
      </div>
    );
  }

  return (
    <div
      className={`fixed top-0 left-0 w-full h-12 flex items-center justify-between pl-6 pr-0 z-50 select-none transition-all ${baseClasses}`}
      data-tauri-drag-region
    >
      <div className="flex items-center text-sm font-semibold text-foreground/90 space-x-3">
        <span>{appInfo.name.toUpperCase()}</span>
      </div>
      <div className="flex items-center ml-auto h-full pr-0">
        <SettingsButton className="mr-3 px-3 py-2" />
        <WindowControls />
      </div>
    </div>
  );
}
