import { useState, useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';
import { useNavigate } from 'react-router-dom';
import BooleanBadge from '../components/BooleanBadge';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '@/components/ui/tooltip';

const TruncatedId = ({ id }) => {
  const [copied, setCopied] = useState(false);

  const shouldTruncate = id.length > 20;
  const displayId = shouldTruncate ? `${id.substring(0, 17)}...` : id;

  const handleCopy = (e) => {
    e.stopPropagation(); // Prevent row click
    navigator.clipboard.writeText(id);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!shouldTruncate) {
    return <span>{id}</span>;
  }

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="cursor-help border-b border-dotted border-muted-foreground/50">
            {displayId}
          </span>
        </TooltipTrigger>
        <TooltipContent
          className="z-50 min-w-[300px] max-w-[600px] rounded-md border border-brand-sky bg-background p-4 shadow-lg shadow-brand-sky/20 flex flex-col gap-2"
          sideOffset={5}
        >
          <div className="break-all text-sm font-mono text-muted-foreground">{id}</div>
          <Button
            size="sm"
            variant="outline"
            className={cn(
              'h-6 text-xs w-fit',
              copied ? 'border-success text-success' : 'border-brand-sky text-brand-sky',
            )}
            onClick={handleCopy}
          >
            {copied ? 'Copied!' : 'Copy'}
          </Button>
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};

const Minions = () => {
  const [minions, setMinions] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [filters, setFilters] = useState({
    id: '',
    slug: '',
    type: '',
    issuedVC: '',
    savedAt: '',
    isMe: '',
  });
  const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
  const navigate = useNavigate();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  const fetchMinions = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/minions/all`);
      if (!response.ok) {
        throw new Error('Failed to fetch minions');
      }
      const data = await response.json();
      setMinions(data);
      setLoading(false);
    } catch (err) {
      console.error('Error fetching minions:', err);
      setError(err);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMinions();
  }, [apiUrl]);

  const handleRowClick = (id) => {
    navigate(`/minions/${id}`);
  };

  const formatDate = (dateString) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleString();
  };

  const handleFilterChange = (field, value) => {
    setFilters((prev) => ({ ...prev, [field]: value }));
  };

  const handleSort = (key) => {
    let direction = 'asc';
    if (sortConfig.key === key && sortConfig.direction === 'asc') {
      direction = 'desc';
    }
    setSortConfig({ key, direction });
  };

  const getSortValue = (minion, key) => {
    switch (key) {
      case 'id':
        return minion.participant_id;
      case 'slug':
        return minion.participant_slug;
      case 'type':
        return minion.participant_type;
      case 'issuedVC':
        return minion.is_vc_issued ? 'Yes' : 'No';
      case 'savedAt':
        return minion.saved_at;
      case 'isMe':
        return minion.is_me ? 'Yes' : 'No';
      default:
        return '';
    }
  };

  const filteredMinions = minions.filter((minion) => {
    const issuedVCText = minion.is_vc_issued ? 'yes' : 'no';
    const isMeText = minion.is_me ? 'yes' : 'no';

    return (
      minion.participant_id.toLowerCase().includes(filters.id.toLowerCase()) &&
      minion.participant_slug.toLowerCase().includes(filters.slug.toLowerCase()) &&
      minion.participant_type.toLowerCase().includes(filters.type.toLowerCase()) &&
      issuedVCText.includes(filters.issuedVC.toLowerCase()) &&
      minion.saved_at.toLowerCase().includes(filters.savedAt.toLowerCase()) &&
      isMeText.includes(filters.isMe.toLowerCase())
    );
  });

  const sortedMinions = [...filteredMinions].sort((a, b) => {
    if (!sortConfig.key) return 0;

    const aValue = getSortValue(a, sortConfig.key);
    const bValue = getSortValue(b, sortConfig.key);

    if (aValue < bValue) {
      return sortConfig.direction === 'asc' ? -1 : 1;
    }
    if (aValue > bValue) {
      return sortConfig.direction === 'asc' ? 1 : -1;
    }
    return 0;
  });

  const getSortIndicator = (key) => {
    if (sortConfig.key !== key) {
      return ' ⇅';
    }
    return sortConfig.direction === 'asc' ? ' ▲' : ' ▼';
  };

  if (loading) return <div className="p-8 text-brand-sky">Loading...</div>;
  if (error) return <GeneralErrorComponent error={error} reset={fetchMinions} />;

  return (
    <div className="w-full">
      <h1 className="text-3xl font-bold text-brand-sky font-ubuntu mb-6">Minions List</h1>
      <div className="rounded-md border border-stroke bg-background/50 shadow-md">
        <Table>
          <TableHeader>
            <TableRow className="border-b-brand-sky/30 hover:bg-transparent">
              <TableHead
                onClick={() => handleSort('id')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                ID{getSortIndicator('id')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('slug')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Slug{getSortIndicator('slug')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('type')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Type{getSortIndicator('type')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('issuedVC')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Issued VC{getSortIndicator('issuedVC')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('savedAt')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Saved At{getSortIndicator('savedAt')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('isMe')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Is Me{getSortIndicator('isMe')}
              </TableHead>
            </TableRow>
            <TableRow className="bg-brand-blue/30 hover:bg-brand-blue/30 border-none">
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.id}
                  onChange={(e) => handleFilterChange('id', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.slug}
                  onChange={(e) => handleFilterChange('slug', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.type}
                  onChange={(e) => handleFilterChange('type', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.issuedVC}
                  onChange={(e) => handleFilterChange('issuedVC', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.savedAt}
                  onChange={(e) => handleFilterChange('savedAt', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.isMe}
                  onChange={(e) => handleFilterChange('isMe', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {sortedMinions.map((minion) => (
              <TableRow
                key={minion.participant_id}
                onClick={() => handleRowClick(minion.participant_id)}
                className="cursor-pointer border-b-brand-sky/20 hover:bg-brand-sky/5 transition-colors"
              >
                <TableCell className="font-mono text-xs">
                  <TruncatedId id={minion.participant_id} />
                </TableCell>
                <TableCell>{minion.participant_slug}</TableCell>
                <TableCell className="text-brand-purple">{minion.participant_type}</TableCell>
                <TableCell>
                  <BooleanBadge value={minion.is_vc_issued} />
                </TableCell>
                <TableCell className="text-muted-foreground">
                  {formatDate(minion.saved_at)}
                </TableCell>
                <TableCell>
                  <BooleanBadge value={minion.is_me} />
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        {sortedMinions.length === 0 && minions.length > 0 && (
          <div className="p-8 text-center text-muted-foreground">
            No minions match the current filters
          </div>
        )}
      </div>
    </div>
  );
};

export default Minions;
