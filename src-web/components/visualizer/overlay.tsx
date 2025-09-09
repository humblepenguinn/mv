import { Cpu } from 'lucide-react';

export function Overlay() {
  return (
    <div className="h-full flex items-center justify-center bg-background overflow-hidden">
      <div className="text-center space-y-4">
        <div className="w-16 h-16 sm:w-20 sm:h-20 mx-auto rounded-full bg-primary/10 flex items-center justify-center">
          <Cpu className="w-6 h-6 sm:w-8 sm:h-8 text-primary/60" />{' '}
        </div>
        <div className="space-y-2">
          <h3 className="text-lg sm:text-xl font-semibold text-foreground">
            Memory Visualization
          </h3>
          <p className="text-sm text-muted-foreground max-w-sm mx-auto leading-relaxed">
            Write some C++ code to see real-time memory visualization of the
            stack and heap allocations'
          </p>
        </div>
      </div>
    </div>
  );
}
