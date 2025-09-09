import React from 'react';

import { createRootRoute, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { Toaster } from 'sonner';
import { AlertCircle, ArrowLeft } from 'lucide-react';

import { appInfo } from '@/lib/appInfo';
import { WindowTitleBar } from '@/components/desktop/window-titlebar';
import { useEditorSettingsStore } from '@/stores/editor';
import { SettingsButton } from '@/components/ui/settings-button';

export const Route = createRootRoute({
  component: () => {
    React.useEffect(() => {
      useEditorSettingsStore.getState().initializeStore();

      function handleTouchMove(e: TouchEvent) {
        e.preventDefault();
      }

      document.addEventListener('touchmove', handleTouchMove);
      return () => {
        document.removeEventListener('touchmove', handleTouchMove);
      };
    }, []);

    return (
      <>
        <Toaster
          position="bottom-center"
          richColors
          className="flex w-full items-center justify-center rounded-lg bg-zinc-800 px-5 py-2 text-zinc-200 shadow-lg"
          offset={{
            top: 15,
          }}
          visibleToasts={1}
          toastOptions={{
            className: '!w-fit !bg-zinc-800 !text-zinc-300 !border-zinc-900',
            style: {
              width: 'fit-content',
              maxWidth: 'fit-content',
              padding: '8px 15px',
            },
          }}
        />
        {appInfo.isDesktop ? (
          <WindowTitleBar />
        ) : (
          <div className="flex items-center justify-between p-4 border-b border-border">
            <div className="flex items-center text-sm font-semibold text-foreground/90 space-x-3">
              <span>{appInfo.name.toUpperCase()}</span>
            </div>
            <SettingsButton className="ml-3 px-2 py-2" />
          </div>
        )}
        <div
          className={`relative min-h-screen flex flex-col w-full bg-background text-foreground ${appInfo.isDesktop ? 'pt-12' : ''}`}
        >
          <div className="flex-1">
            <Outlet />
          </div>
        </div>

        {appInfo.isDev && <TanStackRouterDevtools />}
      </>
    );
  },

  notFoundComponent: () => {
    return (
      <div className="h-screen flex items-center justify-center bg-background overflow-hidden">
        <div className="text-center space-y-6 max-w-md mx-auto px-6">
          <div className="w-20 h-20 mx-auto rounded-full bg-primary/10 flex items-center justify-center">
            <AlertCircle className="w-10 h-10 text-primary/60" />
          </div>
          <div className="space-y-3">
            <h1 className="text-2xl font-semibold text-foreground">
              Page Not Found
            </h1>
            <p className="text-muted-foreground leading-relaxed">
              The page you're looking for does not exist or has been moved
            </p>
          </div>
          <div className="pt-2">
            <button
              onClick={() => window.history.back()}
              className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all bg-primary text-primary-foreground shadow-xs hover:bg-primary/90 h-9 px-4 py-2"
            >
              <ArrowLeft className="w-4 h-4" />
              Go Back
            </button>
          </div>
        </div>
      </div>
    );
  },
});
