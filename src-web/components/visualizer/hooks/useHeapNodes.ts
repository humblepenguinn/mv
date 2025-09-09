import React from 'react';

import { Position } from '@xyflow/react';
import { useTheme } from '@/providers/theme-provider';

import { generateRandomColor } from '@/lib/utils';
import { NODE_WIDTH, HEIGHT_OFFSET } from '../constants';
import {
  getHeightFromSize,
  createEdge,
  calculateNodePosition,
  generateNodeId,
} from '../utils';
import { type NodeData, type EdgeData } from '@/types/visualizer';

interface UseHeapNodesProps {
  analyzeResponse: any;
  stackNodes: NodeData[];
  windowHeight: number;
  heapXCoordinate: number;
  setHeapNodes: (nodes: NodeData[]) => void;
  setHeapConnections: (connections: EdgeData[]) => void;
}

export function useHeapNodes({
  analyzeResponse,
  stackNodes,
  windowHeight,
  heapXCoordinate,
  setHeapNodes,
  setHeapConnections,
}: UseHeapNodesProps) {
  const { theme } = useTheme();

  React.useEffect(() => {
    if (
      !analyzeResponse ||
      !analyzeResponse.heap ||
      analyzeResponse.stack.length === 0
    )
      return;

    const heapNodesInner: NodeData[] = [];
    const connectionsInner: EdgeData[] = [];
    let index = 0;
    let address = 0x00400000;

    for (const block of analyzeResponse.heap) {
      const isFree = block.block_state === 'Free';
      const isUnallocated = block.block_state === 'Unallocated';
      const yPos = calculateNodePosition(
        heapNodesInner[heapNodesInner.length - 1] || null,
        block.size,
        windowHeight,
        HEIGHT_OFFSET
      );

      const label = block.metadata ? block.metadata : 'null';
      const heapNodeID = generateNodeId(index, isUnallocated, isFree);

      const heapNode: NodeData = {
        type: 'memoryBlockNode',
        id: heapNodeID,
        position: { x: heapXCoordinate, y: yPos },
        data: {
          nodeType: 'heap',
          label,
          value: '',
          size: block.size.toString(),
          type: block.block_state === 'Leaked' ? 'LB' : '',
          extraInfo: {
            address: `0x${address.toString(16).toUpperCase()}`,
            isFree,
          },
        },
        width: NODE_WIDTH,
        height: getHeightFromSize(block.size),
        size: block.size,
      };

      // create connections to stack nodes
      if (!heapNodeID.includes('unallocated')) {
        stackNodes.forEach((stackNode) => {
          let stroke = generateRandomColor(theme);
          const isCurrentPointer =
            stackNode.id === block.current_pointer_identifier;
          const isDanglingPointer =
            block.dangling_pointer_identifiers?.includes(stackNode.id);

          if (isCurrentPointer || isDanglingPointer) {
            stackNode.data.extraInfo.pointingToAddress =
              heapNode.data.extraInfo.address;
            heapNode.targetPosition = Position.Left;

            if (isCurrentPointer) {
              // preserve existing stroke color if connection already exists
              connectionsInner.forEach((connection) => {
                if (
                  connection.source === stackNode.id &&
                  connection.target === heapNodeID.toString()
                ) {
                  stroke = connection.style.stroke;
                }
              });
            }

            if (isDanglingPointer) {
              stackNode.data.extraInfo.metadata = 'Dangling Pointer';
              stroke = 'red';
            }

            connectionsInner.push(
              createEdge(
                `e${stackNode.id}-${heapNodeID.toString()}`,
                'straight',
                stackNode.id,
                heapNodeID.toString(),
                stroke
              )
            );
          }
        });
      }

      heapNodesInner.push(heapNode);
      address += block.size;
      index++;
    }

    setHeapNodes(heapNodesInner);
    setHeapConnections(connectionsInner);
  }, [
    analyzeResponse,
    stackNodes,
    heapXCoordinate,
    theme,
    windowHeight,
    setHeapNodes,
    setHeapConnections,
  ]);
}
