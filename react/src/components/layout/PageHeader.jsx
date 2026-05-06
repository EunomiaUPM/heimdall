import React from 'react';
import Heading from '@/components/ui/heading';
import { cn } from '@/lib/utils';

export function PageHeader({ title, badge, className, children, ...props }) {
  return (
    <header className={cn('pt-2 pb-3 mb-2', className)} {...props}>
      <Heading level="h4" className="flex gap-2 items-center font-display mb-0 text-white/90">
        {title}
        {badge}
      </Heading>
      {children}
    </header>
  );
}
