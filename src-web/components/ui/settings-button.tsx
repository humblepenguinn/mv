import React from 'react';

import {
  SettingsIcon,
  MessageCircleIcon,
  Moon,
  Sun,
  Info,
  Download,
  Loader2,
  CheckCircle,
  AlertCircle,
} from 'lucide-react';

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
  DropdownMenuLabel,
} from '@/components/ui/dropdown-menu';
import { Switch } from '@/components/ui/switch';
import { cn } from '@/lib/utils';
import { useTheme } from '@/providers/theme-provider';
import { appInfo } from '@/lib/appInfo';
import { useUpdates } from '@/hooks/useUpdates';
import { UpdateModal } from '@/components/ui/update/modal';
import { invokeCmd } from '@/lib/tauri';
import { toast } from 'sonner';

type SettingsButtonProps = {
  className?: string;
};

export function SettingsButton(props: SettingsButtonProps) {
  const { className } = props;
  const { theme, setTheme } = useTheme();
  const {
    isUpdateModalOpen,
    isInstalling,
    progress,
    updateCheckStatus,
    checkForUpdates,
    installUpdate,
    closeUpdateModal,
  } = useUpdates();

  const [isCheckingUpdates, setIsCheckingUpdates] = React.useState(false);

  const handleCheckForUpdates = React.useCallback(
    async (e: React.MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();

      if (!appInfo.isDesktop) return;

      setIsCheckingUpdates(true);
      try {
        await checkForUpdates();
      } finally {
        setIsCheckingUpdates(false);
      }
    },
    [checkForUpdates]
  );

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button
            className={cn(
              'p-2 rounded-md hover:bg-accent transition-colors cursor-pointer text-muted-foreground hover:text-foreground',
              className
            )}
          >
            <SettingsIcon className="w-4 h-4" />
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="mr-2 min-w-[220px]" align="end">
          <div className="flex items-center justify-between px-3 py-2 rounded-md">
            <div className="flex items-center gap-2">
              {theme === 'dark' ? (
                <Moon className="h-4 w-4 text-muted-foreground" />
              ) : (
                <Sun className="h-4 w-4 text-muted-foreground" />
              )}
              <span className="text-sm font-medium">Dark mode</span>
            </div>
            <Switch
              checked={theme === 'dark'}
              onCheckedChange={(checked: boolean) =>
                setTheme(checked ? 'dark' : 'light')
              }
              className="data-[state=checked]:bg-primary"
            />
          </div>

          <DropdownMenuSeparator />
          <DropdownMenuLabel className="text-xs font-medium text-muted-foreground tracking-wide uppercase">
            Help us Improve
          </DropdownMenuLabel>
          <DropdownMenuItem
            onSelect={async () => {
              const url = 'https://github.com/humblepenguinn/mv/issues';
              if (appInfo.isDesktop) {
                try {
                  await invokeCmd('cmd_open_url', {
                    url,
                  });
                } catch (error) {
                  toast.error(`Failed to open URL: ${error}`);
                }
              } else {
                window.open(url, '_blank');
              }
            }}
            className="flex items-center gap-2 cursor-pointer"
          >
            <MessageCircleIcon className="w-4 h-4" />
            <span>Give us Feedback</span>
          </DropdownMenuItem>

          <DropdownMenuSeparator />
          {appInfo.isDesktop && (
            <>
              <DropdownMenuLabel className="text-xs font-medium text-muted-foreground tracking-wide uppercase">
                Updates
              </DropdownMenuLabel>
              <div
                onClick={handleCheckForUpdates}
                className={cn(
                  'flex items-center gap-2 cursor-pointer px-3 py-2 text-sm rounded-sm hover:bg-accent hover:text-accent-foreground',
                  isCheckingUpdates && 'opacity-50 cursor-not-allowed'
                )}
              >
                {isCheckingUpdates ? (
                  <Loader2 className="w-4 h-4 animate-spin" />
                ) : (
                  <Download className="w-4 h-4" />
                )}
                <span>
                  {isCheckingUpdates ? 'Checking...' : 'Check for Updates'}
                </span>
              </div>
              {updateCheckStatus && (
                <div className="px-3 py-2">
                  <div
                    className={cn(
                      'text-xs flex items-center gap-2',
                      updateCheckStatus.type === 'success' &&
                        'text-green-600 dark:text-green-400',
                      updateCheckStatus.type === 'info' &&
                        'text-blue-600 dark:text-blue-400',
                      updateCheckStatus.type === 'error' &&
                        'text-red-600 dark:text-red-400'
                    )}
                  >
                    {updateCheckStatus.type === 'success' && (
                      <CheckCircle className="w-3 h-3" />
                    )}
                    {updateCheckStatus.type === 'info' && (
                      <Info className="w-3 h-3" />
                    )}
                    {updateCheckStatus.type === 'error' && (
                      <AlertCircle className="w-3 h-3" />
                    )}
                    <span>{updateCheckStatus.message}</span>
                  </div>
                </div>
              )}
              <DropdownMenuSeparator />
            </>
          )}
          <DropdownMenuLabel className="text-xs font-medium text-muted-foreground tracking-wide uppercase">
            Version Info
          </DropdownMenuLabel>
          <div className="px-3 py-2 text-xs text-muted-foreground">
            <div className="flex items-center gap-2 mb-1">
              <Info className="w-3 h-3" />
              <span>MV v{appInfo.version}</span>
            </div>
          </div>
        </DropdownMenuContent>
      </DropdownMenu>

      {appInfo.isDesktop && (
        <UpdateModal
          isOpen={isUpdateModalOpen}
          onClose={closeUpdateModal}
          progress={progress}
          onInstall={installUpdate}
          isInstalling={isInstalling}
        />
      )}
    </>
  );
}
