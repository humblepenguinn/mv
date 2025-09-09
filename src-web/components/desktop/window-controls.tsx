import React from 'react';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

import { appInfo } from '@/lib/appInfo';

export function WindowControls() {
  const isMac = appInfo.os === 'macos';
  const [maximized, setMaximized] = React.useState(false);

  React.useEffect(() => {
    getCurrentWebviewWindow().isMaximized().then(setMaximized);
    const win = getCurrentWebviewWindow();
    const handler = () => win.isMaximized().then(setMaximized);
    let unlisten: (() => void) | undefined;

    win.onResized(handler).then((fn) => {
      unlisten = fn;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  // Hide custom controls on macOS (system overlay is used)
  if (isMac) {
    return null;
  }

  return (
    <div className="flex gap-0.5 h-full items-center select-none">
      <button
        className="w-12 h-12 flex items-center justify-center rounded-none hover:bg-accent active:bg-accent/80 transition-colors text-muted-foreground hover:text-foreground"
        onClick={() => getCurrentWebviewWindow().minimize()}
        aria-label="Minimize"
        tabIndex={0}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <rect
            x="3"
            y="8"
            width="10"
            height="1.5"
            rx="0.75"
            fill="currentColor"
          />
        </svg>
      </button>

      <button
        className="w-12 h-12 flex items-center justify-center rounded-none hover:bg-accent active:bg-accent/80 transition-colors text-muted-foreground hover:text-foreground"
        onClick={async () => {
          const win = getCurrentWebviewWindow();
          const isMax = await win.isMaximized();
          if (isMax) {
            await win.unmaximize();
            setMaximized(false);
          } else {
            await win.maximize();
            setMaximized(true);
          }
        }}
        aria-label="Maximize"
        tabIndex={0}
      >
        {maximized ? (
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <g fill="none" stroke="currentColor" strokeWidth="1.5">
              <rect x="4.5" y="4.5" width="7" height="7" rx="1.5" />
              <rect x="6.5" y="6.5" width="5" height="5" rx="1" />
            </g>
          </svg>
        ) : (
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <rect
              x="3.5"
              y="3.5"
              width="9"
              height="9"
              rx="2"
              stroke="currentColor"
              strokeWidth="1.5"
              fill="none"
            />
          </svg>
        )}
      </button>

      <button
        className="w-12 h-12 flex items-center justify-center rounded-none hover:bg-destructive/10 active:bg-destructive/20 transition-colors text-muted-foreground hover:text-destructive focus:outline-none"
        onClick={() => getCurrentWebviewWindow().close()}
        aria-label="Close"
        tabIndex={0}
      >
        <svg
          width="16"
          height="16"
          viewBox="0 0 16 16"
          fill="none"
          className="block m-0 p-0"
        >
          <path
            d="M5 5l6 6M11 5l-6 6"
            stroke="currentColor"
            strokeWidth="1.5"
            strokeLinecap="round"
          />
        </svg>
      </button>
    </div>
  );
}
