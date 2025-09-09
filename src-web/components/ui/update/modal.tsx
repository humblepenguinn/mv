import { X, Download } from 'lucide-react';
import { Button } from '@/components/ui/button';
import type { UpdateProgressEvent } from '@/types/updates';
import { UpdateStatus } from './status';
import { UpdateProgress } from './progress';

interface UpdateModalProps {
  isOpen: boolean;
  onClose: () => void;
  progress: UpdateProgressEvent | null;
  onInstall: () => void;
  isInstalling: boolean;
}

export function UpdateModal({
  isOpen,
  onClose,
  progress,
  onInstall,
  isInstalling,
}: UpdateModalProps) {
  if (!isOpen) return null;

  const canInstall =
    !isInstalling &&
    progress?.type !== 'Completed' &&
    progress?.type !== 'Failed';

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="fixed inset-0 bg-black/50" onClick={onClose} />
      <div className="relative bg-background border rounded-lg shadow-lg p-6 w-full max-w-md mx-4">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold">Update Available</h2>
          <Button
            variant="ghost"
            size="icon"
            onClick={onClose}
            disabled={isInstalling}
          >
            <X className="w-4 h-4" />
          </Button>
        </div>

        <div className="space-y-4">
          <UpdateStatus progress={progress} isInstalling={isInstalling} />
          <UpdateProgress progress={progress} />

          <div className="flex gap-2 pt-2">
            {canInstall && (
              <Button onClick={onInstall} className="flex-1">
                <Download className="w-4 h-4 mr-2" />
                Install Update
              </Button>
            )}
            <Button
              variant="outline"
              onClick={onClose}
              disabled={isInstalling}
              className="flex-1"
            >
              {isInstalling ? 'Installing...' : 'Later'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
