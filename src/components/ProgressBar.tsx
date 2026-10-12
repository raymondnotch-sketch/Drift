interface ProgressBarProps {
  value: number;
  max: number;
  className?: string;
}

export function ProgressBar({ value, max, className = "" }: ProgressBarProps) {
  const percentage = max > 0 ? (value / max) * 100 : 0;
  
  return (
    <div className={`w-full bg-slate-100 rounded-full h-3 overflow-hidden ${className}`}>
      <div
        className="bg-gradient-to-r from-indigo-500 to-indigo-600 h-3 rounded-full transition-all duration-300"
        style={{ width: `${Math.min(100, Math.max(0, percentage))}%` }}
      />
    </div>
  );
}
