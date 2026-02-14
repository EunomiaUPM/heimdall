import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Input } from '@/components/ui/input';
import { cn } from '@/lib/utils';

const StatusBadge = ({ status, isVcIssued }) => {
  const getStatusClasses = (status, isVcIssued) => {
    switch (status?.toLowerCase()) {
      case 'processing':
      case 'proccesing':
        return 'bg-yellow-500/15 text-yellow-500 border-yellow-500';
      case 'pending':
        return 'bg-orange-500/15 text-orange-500 border-orange-500';
      case 'approved':
        return 'bg-brand-sky/15 text-brand-sky border-brand-sky';
      case 'finalized':
        return isVcIssued
          ? 'bg-green-500/15 text-green-500 border-green-500'
          : 'bg-red-500/15 text-red-500 border-red-500';
      default:
        return 'bg-brand-sky/15 text-brand-sky border-brand-sky';
    }
  };

  return (
    <span
      className={cn(
        'inline-block px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider border shadow-sm',
        getStatusClasses(status, isVcIssued),
      )}
    >
      {status}
    </span>
  );
};

const Requests = () => {
  const [requests, setRequests] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [filters, setFilters] = useState({
    id: '',
    slug: '',
    vcType: '',
    interactMethod: '',
    status: '',
    createdAt: '',
  });
  const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
  const navigate = useNavigate();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchRequests = async () => {
      try {
        const response = await fetch(`${apiUrl}/approver/all`);
        if (!response.ok) {
          throw new Error('Failed to fetch requests');
        }
        const data = await response.json();
        setRequests(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching requests:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchRequests();
  }, [apiUrl]);

  const handleRowClick = (id) => {
    navigate(`/requests/${id}`);
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

  const getSortValue = (req, key) => {
    switch (key) {
      case 'id':
        return req.id;
      case 'slug':
        return req.participant_slug;
      case 'vcType':
        return req.vc_type;
      case 'interactMethod':
        return req.interact_method.join(', ');
      case 'status':
        return req.status;
      case 'createdAt':
        return req.created_at;
      default:
        return '';
    }
  };

  const filteredRequests = requests.filter((req) => {
    return (
      req.id.toLowerCase().includes(filters.id.toLowerCase()) &&
      req.participant_slug.toLowerCase().includes(filters.slug.toLowerCase()) &&
      req.vc_type.toLowerCase().includes(filters.vcType.toLowerCase()) &&
      req.interact_method.join(', ').toLowerCase().includes(filters.interactMethod.toLowerCase()) &&
      req.status.toLowerCase().includes(filters.status.toLowerCase()) &&
      req.created_at.toLowerCase().includes(filters.createdAt.toLowerCase())
    );
  });

  const sortedRequests = [...filteredRequests].sort((a, b) => {
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
  if (error) return <div className="p-8 text-danger">Error: {error}</div>;

  return (
    <div className="w-full">
      <h1 className="text-3xl font-bold text-brand-sky font-ubuntu mb-6">Requests</h1>
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
                onClick={() => handleSort('vcType')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                VC Type{getSortIndicator('vcType')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('interactMethod')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Interact Method{getSortIndicator('interactMethod')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('status')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Status{getSortIndicator('status')}
              </TableHead>
              <TableHead
                onClick={() => handleSort('createdAt')}
                className="cursor-pointer text-brand-sky hover:text-brand-sky/80"
              >
                Created At{getSortIndicator('createdAt')}
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
                  value={filters.vcType}
                  onChange={(e) => handleFilterChange('vcType', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.interactMethod}
                  onChange={(e) => handleFilterChange('interactMethod', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.status}
                  onChange={(e) => handleFilterChange('status', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
              <TableHead className="p-2">
                <Input
                  placeholder="Filter..."
                  value={filters.createdAt}
                  onChange={(e) => handleFilterChange('createdAt', e.target.value)}
                  onClick={(e) => e.stopPropagation()}
                  className="h-8"
                />
              </TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {sortedRequests.map((req) => (
              <TableRow
                key={req.id}
                onClick={() => handleRowClick(req.id)}
                className="cursor-pointer border-b-brand-sky/20 hover:bg-brand-sky/5 transition-colors"
              >
                <TableCell className="font-mono text-xs">{req.id}</TableCell>
                <TableCell>{req.participant_slug}</TableCell>
                <TableCell className="text-brand-purple">{req.vc_type}</TableCell>
                <TableCell>{req.interact_method.join(', ')}</TableCell>
                <TableCell>
                  <StatusBadge status={req.status} isVcIssued={req.is_vc_issued} />
                </TableCell>
                <TableCell>{req.created_at}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
        {sortedRequests.length === 0 && requests.length > 0 && (
          <div className="p-8 text-center text-muted-foreground">
            No requests match the current filters
          </div>
        )}
      </div>
    </div>
  );
};

export default Requests;
