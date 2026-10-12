import { ArrowLeft } from "lucide-react";
import { ReactNode } from "react";

interface ScreenHeaderProps {
  title: string;
  description?: string;
  onBack?: () => void;
  children?: ReactNode;
}

export function ScreenHeader({ title, description, onBack, children }: ScreenHeaderProps) {
  return (
    <div className="mb-8">
      {onBack && (
        <button
          onClick={onBack}
          className="mb-6 flex items-center gap-2 text-slate-600 hover:text-slate-900 transition-colors duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-400 rounded px-2 py-1 -ml-2"
        >
          <ArrowLeft className="w-5 h-5" />
          <span className="font-medium">Back</span>
        </button>
      )}
      <div>
        <h1 className="text-4xl font-bold text-slate-900 mb-2">{title}</h1>
        {description && <p className="text-slate-600">{description}</p>}
        {children}
      </div>
    </div>
  );
}
