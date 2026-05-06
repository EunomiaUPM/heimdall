import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva } from 'class-variance-authority';
import { cn } from '@/lib/utils';

const normalizeStatus = (status) => {
  switch ((status || '').toLowerCase()) {
    case 'active':
    case 'accepted':
    case 'verified':
    case 'started':
    case 'approved':
    case 'agreed':
      return 'process';
    case 'offered':
    case 'requested':
    case 'pending':
    case 'processing':
      return 'warn';
    case 'finalized':
    case 'completed':
      return 'success';
    case 'inactive':
    case 'suspended':
    case 'pause':
    case 'by_provider':
    case 'by_consumer':
    case 'on_request':
    case 'stop':
    case 'stopped':
      return 'pause';
    case 'terminated':
    case 'rejected':
      return 'danger';
    default:
      return 'default';
  }
};

const badgeVariants = cva(
  'px-1.5 py-0.5 w-fit inline-flex justify-start items-center bg-white/5 font-medium rounded-[4px] border border-white/10 whitespace-nowrap shrink-0 gap-1 transition-all',
  {
    variants: {
      variant: {
        default: 'bg-brand-snow/15 text-brand-snow border-white/10',
        info: 'font-mono uppercase bg-background-800 text-secondary-400 border-white/10',
        infoLighter: 'font-mono uppercase bg-white/10 text-secondary-400 border-white/10',
        role: 'text-white uppercase border-white/10',
        status: 'bg-opacity-30 border-white/10 text-foreground-300 uppercase',
        detail: 'text-xs bg-brand-sky/20 !px-1 !py-0 max-w-[140px] !whitespace-normal',
        code: 'bg-gray-900 border border-gray-800 rounded-sm font-mono text-red-500 !py-0',
      },
      state: {
        default: '',
        process: 'bg-process text-process-300 [&>span]:bg-process-400',
        warn: 'bg-warn text-warn-300 [&>span]:bg-warn-400',
        success: 'bg-success text-success-300 [&>span]:bg-success-400',
        pause: 'bg-pause text-pause-300 [&>span]:bg-pause-400',
        danger: 'bg-danger text-danger-300 [&>span]:bg-danger-400',
      },
      dsrole: {
        Provider: 'bg-roles-provider/30 border-roles-provider/40',
        Consumer: 'bg-roles-consumer/30 border-roles-consumer/40',
        Business: 'bg-roles-bussiness/30 border-roles-bussiness/40',
        Customer: 'bg-roles-customer/30 border-roles-customer/40',
      },
      size: {
        default: 'text-xs px-2 py-0.5',
        lg: 'text-sm font-bold px-2 py-1',
        sm: 'text-[10px] px-1.5 py-0.5 leading-none',
      },
    },
    defaultVariants: {
      variant: 'default',
      state: 'default',
      size: 'default',
    },
  },
);

function Badge({
  className,
  variant,
  state,
  size,
  dsrole,
  asChild = false,
  children,
  ...props
}) {
  const Comp = asChild ? Slot : 'span';
  const showDot = variant === 'status';
  const stateStyle = normalizeStatus(state);
  return (
    <Comp
      data-slot="badge"
      className={cn(badgeVariants({ variant, size, state: stateStyle, dsrole }), className)}
      {...props}
    >
      {showDot && <span className={cn('w-2 h-2 rounded-full mr-1 mb-[2px]')} />}
      {children}
    </Comp>
  );
}

export { Badge, badgeVariants };
