interface LabelNodeProps {
  data: {
    label: string;
  };
}

export function LabelNode({ data }: LabelNodeProps) {
  return (
    <div className="font-bold text-base text-foreground bg-card/90 dark:bg-card/95 px-2 py-1 rounded border border-border pointer-events-none text-center min-w-[60px] backdrop-blur-sm">
      {data.label}
    </div>
  );
}
