import React from 'react';

import { ReactFlow } from '@xyflow/react';
import { useWindowSize } from 'react-use';
import { Circle } from 'lucide-react';

import { MemoryBlockNode } from './memory-block-node';
import { LabelNode } from './label-node';
import { useMemoryState } from './hooks/useMemoryState';
import { usePositionState } from './hooks/usePositionState';
import { useStackNodes } from './hooks/useStackNodes';
import { useHeapNodes } from './hooks/useHeapNodes';
import { NODE_WIDTH } from './constants';
import { Overlay } from './overlay';

export interface VisualizerProps {
  analyzeResponse: any;
  isAnalyzing: boolean;
  visualizerPanelSize: number;
  sourceCode: string;
  analyzeError: any;
}

export function Visualizer({
  analyzeResponse,
  isAnalyzing,
  visualizerPanelSize,
  sourceCode,
  analyzeError,
}: VisualizerProps) {
  const windowSize = useWindowSize();

  const memoryState = useMemoryState();
  const positionState = usePositionState(visualizerPanelSize);

  const hasValidAnalyzeResponse =
    analyzeResponse &&
    !analyzeError &&
    (analyzeResponse.stack?.length > 0 || analyzeResponse.heap?.length > 0);

  const shouldShowWriteCodeMessage =
    !sourceCode.trim() && !memoryState.lastValidAnalyzeResponse;

  const shouldShowLastValidVisualization =
    sourceCode.trim() &&
    !hasValidAnalyzeResponse &&
    memoryState.lastValidAnalyzeResponse;

  React.useEffect(() => {
    if (hasValidAnalyzeResponse) {
      memoryState.updateLastValidAnalyzeResponse(analyzeResponse);
    }
  }, [hasValidAnalyzeResponse, analyzeResponse, memoryState]);

  React.useEffect(() => {
    if (
      hasValidAnalyzeResponse &&
      analyzeResponse !== memoryState.lastValidAnalyzeResponse
    ) {
      memoryState.resetMemory();
    }
  }, [hasValidAnalyzeResponse, analyzeResponse, memoryState]);

  const currentAnalyzeResponse = shouldShowLastValidVisualization
    ? memoryState.lastValidAnalyzeResponse
    : analyzeResponse;

  const memoizedAnalyzeResponse = React.useMemo(
    () => currentAnalyzeResponse,
    [currentAnalyzeResponse]
  );

  useStackNodes({
    analyzeResponse: memoizedAnalyzeResponse,
    windowHeight: windowSize.height,
    stackXCoordinate: positionState.stackXCoordinate,
    setStackNodes: memoryState.setStackNodes,
    setStackConnections: memoryState.setStackConnections,
  });

  useHeapNodes({
    analyzeResponse: memoizedAnalyzeResponse,
    stackNodes: memoryState.stackNodes,
    windowHeight: windowSize.height,
    heapXCoordinate: positionState.heapXCoordinate,
    setHeapNodes: memoryState.setHeapNodes,
    setHeapConnections: memoryState.setHeapConnections,
  });

  const labelNodes = React.useMemo(() => {
    const labels = [];

    if (memoryState.stackNodes.length > 0) {
      const topStackNode = memoryState.stackNodes.reduce(
        (topNode, node) =>
          node.position.y < topNode.position.y ? node : topNode,
        memoryState.stackNodes[0]
      );

      labels.push({
        id: 'stack-label',
        type: 'labelNode',
        position: {
          x: positionState.stackXCoordinate + NODE_WIDTH / 2 - 30,
          y: topStackNode.position.y - 50,
        },
        data: { label: 'Stack' },
        draggable: false,
        selectable: false,
      });
    }

    if (memoryState.heapNodes.length > 0) {
      const topHeapNode = memoryState.heapNodes.reduce(
        (topNode, node) =>
          node.position.y < topNode.position.y ? node : topNode,
        memoryState.heapNodes[0]
      );

      labels.push({
        id: 'heap-label',
        type: 'labelNode',
        position: {
          x: positionState.heapXCoordinate + NODE_WIDTH / 2 - 30,
          y: topHeapNode.position.y - 50,
        },
        data: { label: 'Heap' },
        draggable: false,
        selectable: false,
      });
    }

    return labels;
  }, [
    memoryState.stackNodes,
    memoryState.heapNodes,
    positionState.stackXCoordinate,
    positionState.heapXCoordinate,
  ]);

  const nodeTypes = {
    memoryBlockNode: MemoryBlockNode,
    labelNode: LabelNode,
  };

  const hasNoMemoryData =
    memoryState.stackNodes.length === 0 && memoryState.heapNodes.length === 0;

  const shouldShowOverlay =
    shouldShowWriteCodeMessage ||
    (hasNoMemoryData && !shouldShowLastValidVisualization && !isAnalyzing);

  return (
    <div className="h-screen w-full bg-background flex flex-col">
      <div className="px-3 sm:px-4 py-2 sm:py-3 border-b border-border bg-muted/30 flex items-center justify-between flex-shrink-0">
        <h2 className="text-xs sm:text-sm font-semibold text-foreground">
          Memory Visualization
        </h2>
        <div className="flex items-center gap-1 sm:gap-2">
          <Circle className="w-3 h-3 text-green-500 fill-green-500" />
          <span className="text-xs text-muted-foreground flex items-center gap-1">
            Live
          </span>
        </div>
      </div>
      <div className="flex-1 min-h-0">
        <ReactFlow
          nodeTypes={nodeTypes}
          nodes={[
            ...memoryState.stackNodes,
            ...memoryState.heapNodes,
            ...labelNodes,
          ]}
          edges={[
            ...memoryState.stackConnections,
            ...memoryState.heapConnections,
          ]}
          nodeDragThreshold={0}
          edgesFocusable={false}
          nodesDraggable
          nodesConnectable={false}
          nodesFocusable={false}
          draggable={true}
          panOnDrag={true}
          elementsSelectable={false}
          zoomOnScroll={true}
          zoomOnPinch={true}
          zoomOnDoubleClick={false}
          proOptions={{ hideAttribution: true }}
        >
          {shouldShowOverlay && <Overlay />}
        </ReactFlow>
      </div>
    </div>
  );
}
