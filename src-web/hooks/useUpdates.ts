import React from 'react';

import { listen } from '@tauri-apps/api/event';

import { invokeCmd } from '@/lib/tauri';
import { appInfo } from '@/lib/appInfo';
import type {
  UpdateCheckStatus,
  UpdateState,
  UpdateProgressEvent,
} from '@/types/updates';

export function useUpdates() {
  const [updateState, setUpdateState] = React.useState<UpdateState>({
    isUpdateAvailable: false,
    isUpdateModalOpen: false,
    isInstalling: false,
    progress: null,
    updateCheckStatus: null,
  });

  React.useEffect(() => {
    if (!appInfo.isDesktop) return;

    const unlistenUpdateAvailable = listen('update-available', (event) => {
      const updateAvailable = event.payload as boolean;
      setUpdateState((prev) => ({
        ...prev,
        isUpdateAvailable: updateAvailable,
        isUpdateModalOpen: updateAvailable,
      }));
    });

    const unlistenUpdateProgress = listen('update-progress', (event) => {
      const progressEvent = event.payload as UpdateProgressEvent;

      setUpdateState((prev) => ({
        ...prev,
        progress: progressEvent,
        isInstalling:
          progressEvent.type === 'Started' ||
          (prev.isInstalling &&
            progressEvent.type !== 'Completed' &&
            progressEvent.type !== 'Failed'),
      }));
    });

    return () => {
      unlistenUpdateAvailable.then((fn) => fn());
      unlistenUpdateProgress.then((fn) => fn());
    };
  }, []);

  const checkForUpdates = React.useCallback(async () => {
    if (!appInfo.isDesktop) return false;

    try {
      const result = await invokeCmd<boolean>('cmd_check_for_updates');

      const status: UpdateCheckStatus = result
        ? { type: 'success', message: 'Update available!' }
        : { type: 'info', message: 'You are up to date!' };

      setUpdateState((prev) => ({
        ...prev,
        isUpdateAvailable: result,
        isUpdateModalOpen: result,
        updateCheckStatus: status,
      }));

      setTimeout(() => {
        setUpdateState((prev) => ({ ...prev, updateCheckStatus: null }));
      }, 3000);

      return result;
    } catch (error) {
      setUpdateState((prev) => ({
        ...prev,
        updateCheckStatus: {
          type: 'error',
          message: 'Failed to check for updates',
        },
      }));
      setTimeout(() => {
        setUpdateState((prev) => ({ ...prev, updateCheckStatus: null }));
      }, 3000);
      return false;
    }
  }, []);

  const installUpdate = React.useCallback(async () => {
    if (!appInfo.isDesktop) return;

    try {
      setUpdateState((prev) => ({
        ...prev,
        isInstalling: true,
        progress: { type: 'Started' },
      }));
      const result = await invokeCmd<boolean>(
        'cmd_download_and_install_update'
      );
      if (!result) {
        setUpdateState((prev) => ({
          ...prev,
          progress: {
            type: 'Failed',
            data: { message: 'Failed to install update' },
          },
          isInstalling: false,
        }));
        return;
      }
    } catch (error) {
      setUpdateState((prev) => ({
        ...prev,
        progress: {
          type: 'Failed',
          data: {
            message: error instanceof Error ? error.message : 'Unknown error',
          },
        },
        isInstalling: false,
      }));
    }
  }, []);

  const closeUpdateModal = React.useCallback(() => {
    setUpdateState((prev) => ({
      ...prev,
      isUpdateModalOpen: false,
      progress: null,
      isInstalling: false,
    }));
  }, []);

  return {
    ...updateState,
    checkForUpdates,
    installUpdate,
    closeUpdateModal,
  };
}
