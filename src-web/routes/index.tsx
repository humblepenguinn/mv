import React from 'react';
import { createFileRoute } from '@tanstack/react-router';

import { Editor } from '@/components/monaco/editor';
import { Visualizer } from '@/components/visualizer';
import {
  ResizablePanelGroup,
  ResizablePanel,
  ResizableHandle,
} from '@/components/ui/resizable';
import { useAnalyzeSourceCode } from '@/hooks/useAnalyzeSourceCode';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  const [sourceCode, setSourceCode] = React.useState('');
  const [visualizerPanelSize, setVisualizerPanelSize] = React.useState(50);

  const {
    data: analyzeResponse,
    isLoading: isAnalyzing,
    error: analyzeError,
  } = useAnalyzeSourceCode(sourceCode);

  return (
    <div className="h-screen flex flex-col overflow-hidden">
      <ResizablePanelGroup direction="horizontal" className="flex-1 min-h-0">
        <ResizablePanel defaultSize={50} minSize={25} maxSize={75}>
          <div className="h-full flex flex-col border-r border-border bg-background">
            <div className="flex-1 min-h-0 overflow-auto">
              <Editor
                code={sourceCode}
                onChange={setSourceCode}
                analyzeError={analyzeError}
              />
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle
          withHandle
          className="w-1 bg-border hover:bg-border/80 transition-colors flex-shrink-0"
        />
        <ResizablePanel
          defaultSize={visualizerPanelSize}
          minSize={25}
          maxSize={75}
          onResize={setVisualizerPanelSize}
        >
          <div className="h-full overflow-hidden">
            <Visualizer
              analyzeResponse={analyzeResponse ?? { stack: [], heap: [] }}
              isAnalyzing={isAnalyzing}
              visualizerPanelSize={visualizerPanelSize}
              sourceCode={sourceCode}
              analyzeError={analyzeError}
            />
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}
