import { MarkerType } from '@xyflow/react';
import { BASE_NODE_HEIGHT } from './constants';
import { type NodeData, type EdgeData } from '@/types/visualizer';

export function getHeightFromSize(size: number): number {
  return size * BASE_NODE_HEIGHT;
}

export function createEdge(
  id: string,
  type: string,
  source: string,
  target: string,
  color: string
): EdgeData {
  return {
    id,
    type,
    source,
    target,
    markerEnd: {
      type: MarkerType.ArrowClosed,
      width: 20,
      height: 10,
      color,
    },
    style: { strokeWidth: 4, stroke: color },
  };
}

export function calculateNodePosition(
  previousNode: NodeData | null,
  nodeSize: number,
  windowHeight: number,
  heightOffset: number
): number {
  if (!previousNode) {
    return windowHeight - getHeightFromSize(nodeSize) - heightOffset;
  }
  return previousNode.position.y - getHeightFromSize(nodeSize);
}

export function generateNodeId(
  index: number,
  isUnallocated: boolean,
  isFree: boolean
): string {
  if (isUnallocated) {
    return `unallocated-${index}`;
  }
  return isFree ? `free-${index}` : index.toString();
}

export function calculateMemoryUsage(nodes: NodeData[]): number {
  return nodes.reduce((acc, node) => {
    if (node.id.includes('free') || node.id.includes('unallocated')) {
      return acc;
    }
    return acc + node.size;
  }, 0);
}
