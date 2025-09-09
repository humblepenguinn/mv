import { Download, CheckCircle, AlertCircle, Loader2 } from 'lucide-react';
import type { UpdateProgressEvent } from '@/types/updates';

interface UpdateStatusProps {
  progress: UpdateProgressEvent | null;
  isInstalling: boolean;
}

export function UpdateStatus({ progress, isInstalling }: UpdateStatusProps) {
  const getStatusIcon = () => {
    if (isInstalling) {
      return <Loader2 className="w-5 h-5 animate-spin" />;
    }
    if (progress?.type === 'Completed') {
      return <CheckCircle className="w-5 h-5 text-green-500" />;
    }
    if (progress?.type === 'Failed') {
      return <AlertCircle className="w-5 h-5 text-red-500" />;
    }
    return <Download className="w-5 h-5" />;
  };

  const getStatusText = () => {
    if (isInstalling) {
      switch (progress?.type) {
        case 'Started':
          return 'Preparing update...';
        case 'Downloading':
          return `Downloading update... ${progress.data?.percentage || 0}%`;
        case 'Installing':
          return 'Installing update...';
        case 'Completed':
          return 'Update completed! Restarting...';
        case 'Failed':
          return `Update failed: ${progress.data?.message || 'Unknown error'}`;
        default:
          return 'Updating...';
      }
    } else if (progress?.type === 'Failed') {
      return `Update failed: ${progress.data?.message || 'Unknown error'}`;
    }

    return 'Update available';
  };

  return (
    <div className="flex items-center gap-3">
      {getStatusIcon()}
      <span className="text-sm">{getStatusText()}</span>
    </div>
  );
}
