import { ButtonHTMLAttributes, forwardRef } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "ghost" | "danger";
  size?: "default" | "large";
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant = "primary", size = "default", className = "", children, ...props }, ref) => {
    const baseClasses = "rounded-xl font-semibold transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer";
    
    const variantClasses = {
      primary: "bg-indigo-600 text-white hover:bg-indigo-700 focus-visible:ring-indigo-500 active:bg-indigo-800",
      secondary: "bg-white border border-slate-200 text-slate-700 hover:bg-slate-50 focus-visible:ring-slate-400 active:bg-slate-100",
      ghost: "text-slate-600 hover:text-slate-900 hover:bg-slate-100 focus-visible:ring-slate-400",
      danger: "bg-red-50 border border-red-200 text-red-700 hover:bg-red-100 focus-visible:ring-red-500 active:bg-red-200"
    };
    
    const sizeClasses = {
      default: "px-6 py-3",
      large: "px-8 py-4 min-h-[44px]"
    };
    
    return (
      <button
        ref={ref}
        className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
        {...props}
      >
        {children}
      </button>
    );
  }
);

Button.displayName = "Button";
