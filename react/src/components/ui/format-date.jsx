import React from 'react';

// Simple "DD/MM/YYYY - HH:mm" formatter. Heimdall doesn't have dayjs installed
// so we use Intl/native Date.
const pad = (n) => String(n).padStart(2, '0');

export const FormatDate = ({ date }) => {
  if (!date) return <span className="text-gray-400">-</span>;
  const d = date instanceof Date ? date : new Date(date);
  if (isNaN(d.getTime())) return <span className="text-gray-400">-</span>;
  const formatted = `${pad(d.getDate())}/${pad(d.getMonth() + 1)}/${d.getFullYear()} - ${pad(d.getHours())}:${pad(d.getMinutes())}`;
  return <span>{formatted}</span>;
};
