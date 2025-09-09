import React from 'react';

import { type NodeData, type EdgeData } from '@/types/visualizer';

export function useMemoryState() {
  const [stackNodes, setStackNodes] = React.useState<NodeData[]>([]);
  const [heapNodes, setHeapNodes] = React.useState<NodeData[]>([]);
  const [stackConnections, setStackConnections] = React.useState<EdgeData[]>(
    []
  );
  const [heapConnections, setHeapConnections] = React.useState<EdgeData[]>([]);
  const [lastValidAnalyzeResponse, setLastValidAnalyzeResponse] =
    React.useState<any>(null);

  const resetMemory = React.useCallback(() => {
    setStackNodes([]);
    setHeapNodes([]);
    setStackConnections([]);
    setHeapConnections([]);
  }, []);

  const memoizedSetStackNodes = React.useCallback((nodes: NodeData[]) => {
    setStackNodes(nodes);
  }, []);

  const memoizedSetHeapNodes = React.useCallback((nodes: NodeData[]) => {
    setHeapNodes(nodes);
  }, []);

  const memoizedSetStackConnections = React.useCallback(
    (connections: EdgeData[]) => {
      setStackConnections(connections);
    },
    []
  );

  const memoizedSetHeapConnections = React.useCallback(
    (connections: EdgeData[]) => {
      setHeapConnections(connections);
    },
    []
  );

  const updateLastValidAnalyzeResponse = React.useCallback(
    (analyzeResponse: any) => {
      setLastValidAnalyzeResponse(analyzeResponse);
    },
    []
  );

  return {
    stackNodes,
    heapNodes,
    stackConnections,
    heapConnections,
    setStackNodes: memoizedSetStackNodes,
    setHeapNodes: memoizedSetHeapNodes,
    setStackConnections: memoizedSetStackConnections,
    setHeapConnections: memoizedSetHeapConnections,
    resetMemory,
    lastValidAnalyzeResponse,
    updateLastValidAnalyzeResponse,
  };
}
