import { File, X } from "lucide-react";

interface FileRowProps {
  name: string;
  size: string;
  onRemove?: () => void;
}

export function FileRow({ name, size, onRemove }: FileRowProps) {
  return (
    <div className="flex items-center gap-3 p-3 bg-slate-50 rounded-lg border border-slate-200">
      <File className="w-5 h-5 text-slate-400 flex-shrink-0" />
      <div className="flex-1 min-w-0">
        <div className="text-sm font-medium text-slate-900 truncate" title={name}>
          {name}
        </div>
        <div className="text-xs text-slate-500">{size}</div>
      </div>
      {onRemove && (
        <button
          onClick={onRemove}
          className="p-1 text-slate-400 hover:text-slate-600 hover:bg-slate-200 rounded transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-400"
          aria-label="Remove file"
        >
          <X className="w-4 h-4" />
        </button>
      )}
    </div>
  );
}
