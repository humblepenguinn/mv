import { Position } from '@xyflow/react';

export interface NodeData {
  id: string;
  type: string;
  position: { x: number; y: number };
  sourcePosition?: Position;
  targetPosition?: Position;
  data: {
    nodeType: string;
    label: string;
    value: string;
    size: string;
    type?: string;
    extraInfo: {
      address: string;
      pointingToAddress?: string;
      pointingToLabel?: string;
      metadata?: string;
      isFree?: boolean;
    };
  };
  width: number;
  height: number;
  size: number;
  current_pointer_identifier?: any;
  dangling_pointer_identifiers?: any;
}

export interface EdgeData {
  id: string;
  type: string;
  source: string;
  target: string;
  markerEnd: {
    type: any;
    width: number;
    height: number;
    color: string;
  };
  style: { strokeWidth: number; stroke: string };
}

export interface MemoryState {
  stackFull: boolean;
  heapFull: boolean;
  stackNodes: NodeData[];
  heapNodes: NodeData[];
  stackConnections: EdgeData[];
  heapConnections: EdgeData[];
  maxMemory: number;
}

export interface PositionState {
  stackXCoordinate: number;
  heapXCoordinate: number;
  stackLabelPosition: { x: number; y: number };
  heapLabelPosition: { x: number; y: number };
}
