import React from 'react';
import { cn } from '@/lib/utils';

export function InfoGrid({ className, children, ...props }) {
  return (
    <div className={cn('grid grid-cols-1 md:grid-cols-2 gap-3 mb-4', className)} {...props}>
      {children}
    </div>
  );
}
