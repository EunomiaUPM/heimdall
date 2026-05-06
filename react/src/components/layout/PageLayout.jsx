import React from 'react';
import { cn } from '@/lib/utils';

export function PageLayout({ className, children, ...props }) {
  return (
    <div className={cn('space-y-2 pb-2 px-3 w-full', className)} {...props}>
      {children}
    </div>
  );
}
