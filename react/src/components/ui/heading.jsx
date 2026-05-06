import React from 'react';
import { cn } from '@/lib/utils';

const levelToElement = {
  h1: 'h1',
  h2: 'h2',
  h3: 'h3',
  h4: 'h4',
  h5: 'h5',
  h6: 'h6',
  table: 'h6',
  'title-sm': 'h6',
  subtitle: 'h5',
};

const sizeClasses = {
  h1: 'text-2xl mb-4 font-semibold font-title tracking-tight',
  h2: 'text-xl mb-3 font-semibold tracking-tight',
  h3: 'text-lg mb-3 font-medium tracking-tight',
  h4: 'text-base mb-2 font-medium font-display tracking-normal text-white/90',
  h5: 'text-sm text-white/80 mb-2 font-medium tracking-wide',
  h6: 'text-xs font-semibold mb-1 uppercase tracking-wider text-muted-foreground',
  table: 'text-xs font-semibold uppercase tracking-wider',
  'title-sm': 'text-sm font-medium mb-1 leading-snug',
  subtitle: 'text-base mb-2 font-normal text-muted-foreground max-w-[65ch]',
};

const Heading = ({ level = 'h1', children, className = '' }) => {
  const Component = levelToElement[level] || 'h1';
  const baseClasses = 'text-foreground-100 text-balance';
  return (
    <Component className={cn(baseClasses, sizeClasses[level], className)}>{children}</Component>
  );
};

export default Heading;
