import React from 'react';

interface LogoProps {
  className?: string;
  textClassName?: string;
}

export function Logo({ className = '', textClassName = '' }: LogoProps) {
  return (
    <div className={`flex items-center gap-2 ${className}`}>
      <span className="text-brand-blue font-mono">&lt;...&gt;</span>
      <span className={`font-bold ${textClassName}`}>peersend</span>
    </div>
  );
}