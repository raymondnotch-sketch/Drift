import { AlertCircle, CheckCircle, XCircle, AlertTriangle } from "lucide-react";
import { ReactNode } from "react";

interface StatusMessageProps {
  type: "error" | "success" | "warning" | "info";
  title?: string;
  children: ReactNode;
  className?: string;
}

export function StatusMessage({ type, title, children, className = "" }: StatusMessageProps) {
  const styles = {
    error: {
      container: "bg-red-50 border-red-200 text-red-800",
      icon: <XCircle className="w-5 h-5 text-red-600 flex-shrink-0" />
    },
    success: {
      container: "bg-green-50 border-green-200 text-green-800",
      icon: <CheckCircle className="w-5 h-5 text-green-600 flex-shrink-0" />
    },
    warning: {
      container: "bg-amber-50 border-amber-200 text-amber-800",
      icon: <AlertTriangle className="w-5 h-5 text-amber-600 flex-shrink-0" />
    },
    info: {
      container: "bg-blue-50 border-blue-200 text-blue-800",
      icon: <AlertCircle className="w-5 h-5 text-blue-600 flex-shrink-0" />
    }
  };
  
  const style = styles[type];
  
  return (
    <div className={`${style.container} border rounded-xl p-4 flex gap-3 ${className}`}>
      {style.icon}
      <div className="flex-1 text-sm leading-relaxed">
        {title && <div className="font-semibold mb-1">{title}</div>}
        {children}
      </div>
    </div>
  );
}
