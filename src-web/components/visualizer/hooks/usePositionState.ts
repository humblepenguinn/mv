import React from 'react';

import { useWindowSize } from 'react-use';
import { NODE_WIDTH } from '../constants';

export function usePositionState(visualizerPanelSize: number) {
  const windowSize = useWindowSize();
  const [stackXCoordinate, setStackXCoordinate] = React.useState<number>(50);
  const [heapXCoordinate, setHeapXCoordinate] = React.useState<number>(0);
  const [stackLabelPosition, setStackLabelPosition] = React.useState({
    x: stackXCoordinate,
    y: 0,
  });
  const [heapLabelPosition, setHeapLabelPosition] = React.useState({
    x: heapXCoordinate,
    y: 0,
  });

  React.useEffect(() => {
    const newStackXCoordinate = 50;
    const visualizerPanelWidth = (windowSize.width * visualizerPanelSize) / 100;
    const newHeapXCoordinate = visualizerPanelWidth - NODE_WIDTH - 50;

    setStackXCoordinate(newStackXCoordinate);
    setHeapXCoordinate(newHeapXCoordinate);
  }, [visualizerPanelSize, windowSize.width]);

  return {
    stackXCoordinate,
    heapXCoordinate,
    stackLabelPosition,
    heapLabelPosition,
    setStackLabelPosition,
    setHeapLabelPosition,
  };
}
