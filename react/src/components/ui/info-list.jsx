import React from 'react';
import { Badge } from './badge';
import { cn, formatUrn } from '@/lib/utils';
import { FormatDate } from './format-date';

export const InfoList = ({ items, className }) => {
  return (
    <div className={cn('min-w-full px-0', className)}>
      {items.map((item, index) => (
        <InfoListItem key={index} {...item} />
      ))}
    </div>
  );
};

export const InfoListItem = ({ label, value, className, keyClassName }) => {
  if (value === undefined || value === null) return null;

  const renderValue = () => {
    if (typeof value === 'string') {
      return <Badge variant="info">{value}</Badge>;
    }

    if (typeof value === 'object') {
      if ('content' in value) return value.content;

      switch (value.type) {
        case 'date':
          return <FormatDate date={value.value} />;
        case 'status':
          return (
            <Badge variant="status" state={value.value}>
              {value.value}
            </Badge>
          );
        case 'role':
          return (
            <Badge variant="role" dsrole={value.value}>
              {value.value}
            </Badge>
          );
        case 'urn':
          return <Badge variant="info">{formatUrn(value.value)}</Badge>;
        case 'text':
          return <p className="text-sm font-medium text-white/90 capitalize">{value.value}</p>;
        default:
          return null;
      }
    }
    return null;
  };

  return (
    <div className={cn('flex flex-col py-2 border-b border-white/5 last:border-0', className)}>
      <span
        className={cn(
          'text-[10px] uppercase tracking-wide text-white/50 font-medium mb-1',
          keyClassName,
        )}
      >
        {label}
      </span>
      <div className="text-sm font-medium text-white/90">{renderValue()}</div>
    </div>
  );
};
