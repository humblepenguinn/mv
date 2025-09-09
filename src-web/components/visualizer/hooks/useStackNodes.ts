import React from 'react';

import { Position } from '@xyflow/react';
import { useTheme } from '@/providers/theme-provider';
import { generateRandomColor } from '@/lib/utils';
import { NODE_WIDTH, HEIGHT_OFFSET } from '../constants';
import { getHeightFromSize, createEdge, calculateNodePosition } from '../utils';
import { type NodeData, type EdgeData } from '@/types/visualizer';

interface UseStackNodesProps {
  analyzeResponse: any;
  windowHeight: number;
  stackXCoordinate: number;
  setStackNodes: (nodes: NodeData[]) => void;
  setStackConnections: (connections: EdgeData[]) => void;
}

export function useStackNodes({
  analyzeResponse,
  windowHeight,
  stackXCoordinate,
  setStackNodes,
  setStackConnections,
}: UseStackNodesProps) {
  const { theme } = useTheme();

  React.useEffect(() => {
    if (!analyzeResponse || !analyzeResponse.stack) return;

    const stackNodesInner: NodeData[] = [];
    const connectionsInner: EdgeData[] = [];
    let address = 0xbfffffff;

    for (const symbol of analyzeResponse.stack) {
      if (symbol.hasOwnProperty('Variable')) {
        const yPos = calculateNodePosition(
          stackNodesInner[stackNodesInner.length - 1] || null,
          symbol.Variable.size,
          windowHeight,
          HEIGHT_OFFSET
        );

        const stackNode: NodeData = {
          type: 'memoryBlockNode',
          id: symbol.Variable.name,
          position: { x: stackXCoordinate, y: yPos },
          data: {
            nodeType: 'stack',
            label: symbol.Variable.name as string,
            value: symbol.Variable.value
              ? symbol.Variable.value
              : 'Uninitialized',
            size: symbol.Variable.size,
            type: symbol.Variable.vtype,
            extraInfo: {
              address: `0x${address.toString(16).toUpperCase()}`,
            },
          },
          width: NODE_WIDTH,
          height: getHeightFromSize(symbol.Variable.size),
          size: symbol.Variable.size,
        };

        address += symbol.Variable.size;
        stackNodesInner.push(stackNode);
      } else if (symbol.hasOwnProperty('Pointer')) {
        const yPos = calculateNodePosition(
          stackNodesInner[stackNodesInner.length - 1] || null,
          symbol.Pointer.pointer_size,
          windowHeight,
          HEIGHT_OFFSET
        );

        const stackNode: NodeData = {
          type: 'memoryBlockNode',
          id: symbol.Pointer.name,
          position: { x: stackXCoordinate, y: yPos },
          sourcePosition: Position.Right,
          data: {
            nodeType: 'stack',
            label: `*${symbol.Pointer.name}`,
            value: '',
            size: symbol.Pointer.pointer_size,
            type: 'Pointer',
            extraInfo: {
              address: `0x${address.toString(16).toUpperCase()}`,
              pointingToLabel: symbol.Pointer.value?.Variable?.name,
            },
          },
          width: NODE_WIDTH,
          height: getHeightFromSize(symbol.Pointer.pointer_size),
          size: symbol.Pointer.pointer_size,
        };

        address += symbol.Pointer.pointer_size;
        stackNodesInner.push(stackNode);
      }
    }

    // create connections for pointers
    stackNodesInner.forEach((node) => {
      if (node.data.type === 'Pointer') {
        stackNodesInner.forEach((innerNode) => {
          if (innerNode.id === node.data.extraInfo.pointingToLabel) {
            node.data.extraInfo.pointingToAddress =
              innerNode.data.extraInfo.address;
            node.data.value = `&${node.data.extraInfo.pointingToLabel}`;

            innerNode.sourcePosition = Position.Right;
            innerNode.targetPosition = Position.Right;

            const stroke = generateRandomColor(theme);

            connectionsInner.push(
              createEdge(
                `e${node.id}-${innerNode.id}`,
                'step',
                node.id,
                innerNode.id,
                stroke
              )
            );
          }
        });
      }
    });

    setStackNodes(stackNodesInner);
    setStackConnections(connectionsInner);
  }, [
    analyzeResponse,
    stackXCoordinate,
    theme,
    windowHeight,
    setStackNodes,
    setStackConnections,
  ]);
}
