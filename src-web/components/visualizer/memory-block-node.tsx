import React from 'react';
import { LockIcon, UnlockIcon } from 'lucide-react';
import { type NodeProps, type Node, Handle } from '@xyflow/react';

import { BASE_NODE_HEIGHT, NODE_WIDTH } from './constants';
import { useEditorSettingsStore } from '@/stores/editor';
import { useTheme } from '@/providers/theme-provider';

export type MemoryBlockNode = Node<{
  nodeType: string;
  label: string;
  value: string;
  size: string;
  type: string;
  extraInfo: {
    address: string;
    pointingToAddress?: string;
    metadata?: string;
    isFree?: boolean;
  };
}>;

function getColorFromType(type: string, theme?: string): string {
  const lightColors: { [key: string]: string } = {
    Integer: '#8b4513',
    Float: '#f78092',
    Double: '#fb7500',
    Char: '#a31b03',
    Bool: '#118a11',
    Pointer: '#3484da',
    LB: 'red',
  };

  const darkColors: { [key: string]: string } = {
    Integer: '#a76638',
    Float: '#f78092',
    Double: '#fb7500',
    Char: '#ff2600',
    Bool: '#11bd11',
    Pointer: '#3484da',
    LB: 'red',
  };

  const colors = theme === 'dark' ? darkColors : lightColors;

  return colors[type] || '';
}

interface HeapBlockLockIconProps {
  isLocked: boolean;
  style?: React.CSSProperties;
  className?: string;
}

const HeapBlockLockIcon: React.FC<HeapBlockLockIconProps> = ({
  isLocked,
  style,
  className,
}) => {
  const Icon = isLocked ? LockIcon : UnlockIcon;
  const { theme } = useTheme();

  const iconColor = isLocked
    ? theme === 'dark'
      ? '#ef4444'
      : '#dc2626'
    : theme === 'dark'
      ? '#22c55e'
      : '#16a34a';

  return (
    <Icon
      size={20}
      style={{ ...style, color: iconColor }}
      className={className}
    />
  );
};

export function MemoryBlockNode(props: NodeProps<MemoryBlockNode>) {
  const { theme } = useTheme();
  const [showExtraInfo, setShowExtraInfo] = React.useState<boolean>(false);
  const { settings } = useEditorSettingsStore();

  const nodeHeight = props.height || 'auto';
  const isCompact = Number(nodeHeight) <= BASE_NODE_HEIGHT;

  const handleClick = () => {
    setShowExtraInfo(!showExtraInfo);
  };

  const baseStyles = {
    container: {
      position: 'relative' as const,
      padding: '10px',
      height: nodeHeight,
      width: NODE_WIDTH,
      transition: 'width 0.3s ease, height 0.3s ease',
      cursor: 'pointer',
      overflow: 'hidden',
    },
    labelValueWrapper: {
      display: 'flex',
      alignItems: 'center',
      justifyContent: isCompact ? 'space-between' : 'flex-start',
    },
    label: {
      color: getColorFromType(props.data.type, theme),
      fontSize: '17px',
      marginTop: isCompact ? '-7px' : '0',
      marginRight: '8px',
    },
    metadata: {
      marginTop: '5px',
      fontSize: '12px',
    },
    value: {
      fontSize: '14px',
      display: 'block',
      marginRight: isCompact ? '120px' : '0',
      marginTop: isCompact ? '-7px' : '0',
    },
    size: {
      position: 'absolute' as const,
      bottom: '5px',
      right: '5px',
      fontSize: '12px',
    },
    extraData: {
      position: 'absolute' as const,
      bottom: '5px',
      left: isCompact ? '25px' : '5px',
      fontSize: '12px',
      padding: '2px 5px',
      borderRadius: '3px',
      display: showExtraInfo ? 'block' : 'none',
    },
    address: {
      color: theme === 'dark' ? '#fcd66d' : '#fab700',
    },
    lockIcon: {
      marginRight: '20px',
      marginBottom: props.data.extraInfo.isFree && isCompact ? '25px' : 'auto',
    },
  };

  return (
    <div
      style={baseStyles.container}
      className="nodrag bg-card border border-border hover:border-primary/50 transition-colors"
      onClick={handleClick}
    >
      <div style={baseStyles.labelValueWrapper}>
        <div style={baseStyles.label}>{props.data.label}</div>
        {props.data.nodeType === 'heap' &&
          !props.id.includes('unallocated') &&
          !isCompact &&
          settings.showHeapLockIcon && (
            <HeapBlockLockIcon
              isLocked={!props.data.extraInfo.isFree}
              style={baseStyles.lockIcon}
              className="text-foreground"
            />
          )}
        {isCompact && (
          <div style={baseStyles.value} className="text-foreground">
            {' '}
            {props.data.value === 'Uninitialized' || props.data.value === ''
              ? ''
              : `= '${props.data.value}'`}
            {props.data.nodeType === 'heap' &&
              !props.id.includes('unallocated') &&
              settings.showHeapLockIcon && (
                <HeapBlockLockIcon
                  isLocked={!props.data.extraInfo.isFree}
                  style={baseStyles.lockIcon}
                  className="text-foreground"
                />
              )}
          </div>
        )}
      </div>

      {!isCompact && (
        <>
          <div style={baseStyles.value} className="text-foreground">
            {props.data.value}
          </div>
          {props.data.extraInfo.metadata && (
            <div style={baseStyles.metadata} className="text-muted-foreground">
              {props.data.extraInfo.metadata}
            </div>
          )}
        </>
      )}

      {props.sourcePosition && (
        <Handle type="source" position={props.sourcePosition} id="source" />
      )}

      {props.targetPosition && (
        <Handle type="target" position={props.targetPosition} id="target" />
      )}

      <div style={baseStyles.size} className="text-muted-foreground">
        {props.data.size} B
      </div>

      {showExtraInfo && (
        <div
          style={baseStyles.extraData}
          className="bg-muted/80 text-foreground"
        >
          <div style={baseStyles.address}>
            Address: {props.data.extraInfo.address}
          </div>
          {props.data.extraInfo.pointingToAddress && (
            <div className="text-foreground">
              Pointing to: {props.data.extraInfo.pointingToAddress}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
