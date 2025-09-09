export interface UpdateProgressEvent {
  type: 'Started' | 'Downloading' | 'Installing' | 'Completed' | 'Failed';
  data?: {
    progress?: number;
    total?: number;
    percentage?: number;
    message?: string;
  };
}

export interface UpdateCheckStatus {
  type: 'success' | 'info' | 'error';
  message: string;
}

export interface UpdateState {
  isUpdateAvailable: boolean;
  isUpdateModalOpen: boolean;
  isInstalling: boolean;
  progress: UpdateProgressEvent | null;
  updateCheckStatus: UpdateCheckStatus | null;
}
