import React from 'react';
import Heading from '@/components/ui/heading';
import { cn } from '@/lib/utils';

export function PageSection({ title, action, className, children, ...props }) {
  return (
    <div className={cn('mb-6', className)} {...props}>
      <div className="flex items-center justify-start gap-4 mb-2">
        {title && (
          <Heading
            level="h6"
            className="mt-2 text-white/80 uppercase tracking-wide text-xs font-semibold"
          >
            {title}
          </Heading>
        )}
        {action && <div className="mt-1">{action}</div>}
      </div>
      {children}
    </div>
  );
}
