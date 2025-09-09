import { AlertTriangle } from 'lucide-react';

import type { UpdateProgressEvent } from '@/types/updates';

interface UpdateProgressProps {
  progress: UpdateProgressEvent | null;
}

function formatBytesToMB(bytes: number): string {
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}

export function UpdateProgress({ progress }: UpdateProgressProps) {
  const warningComponent = (
    <div className="flex items-center gap-3 p-4 rounded-xl bg-gradient-to-r from-amber-50 to-orange-50 dark:from-amber-950/30 dark:to-orange-950/30 border border-amber-200/50 dark:border-amber-800/30 shadow-sm">
      <div className="flex-shrink-0 w-8 h-8 rounded-full bg-amber-100 dark:bg-amber-900/50 flex items-center justify-center">
        <AlertTriangle className="w-4 h-4 text-amber-600 dark:text-amber-400" />
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-sm font-medium text-amber-900 dark:text-amber-100 leading-5">
          Keep the app open while updating
        </p>
        <p className="text-xs text-amber-700/80 dark:text-amber-300/80 mt-1 leading-4">
          Closing the app may interrupt the update process.
        </p>
      </div>
    </div>
  );

  if (progress?.type === 'Downloading') {
    const percentage = progress.data?.percentage || 0;
    const progressBytes = progress.data?.progress || 0;
    const totalBytes = progress.data?.total;

    const totalMB = totalBytes ? formatBytesToMB(totalBytes) : '?';

    return (
      <div className="space-y-2">
        <div className="flex justify-between text-xs text-muted-foreground">
          <span>
            {formatBytesToMB(progressBytes)} / {totalMB}
          </span>
          <span>{percentage}%</span>
        </div>
        <div className="w-full bg-secondary rounded-full h-2">
          <div
            className="bg-primary h-2 rounded-full transition-all duration-300"
            style={{ width: `${percentage}%` }}
          />
        </div>
        {warningComponent}
      </div>
    );
  }

  if (progress?.type === 'Installing') {
    return (
      <div className="space-y-2">
        <div className="w-full bg-secondary rounded-full h-2">
          <div className="bg-primary h-2 rounded-full animate-pulse" />
        </div>
        {warningComponent}
      </div>
    );
  }

  return null;
}
