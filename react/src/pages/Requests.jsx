import { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { ArrowRight, ArrowUpDown, ArrowUp, ArrowDown } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { formatIdentifier } from '@/lib/utils';
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
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';
import { FormatDate } from '@/components/ui/format-date';

const getIdentityProofDisplay = (methods) => {
  if (!methods || methods.length === 0) return '—';
  if (methods.length === 1 && methods[0] === '') return 'Certificate';
  if (methods.includes('oidc4vp')) return 'Verifiable Credential';
  return methods.join(', ');
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
  const [sortConfig, setSortConfig] = useState(null);
  const navigate = useNavigate();

  const fetchRequests = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/approver/all`);
      if (!response.ok) throw new Error('Failed to fetch requests');
      const data = await response.json();
      setRequests(data);
    } catch (err) {
      console.error('Error fetching requests:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchRequests();
  }, []);

  const handleSort = (key) => {
    let direction = 'asc';
    if (sortConfig && sortConfig.key === key && sortConfig.direction === 'asc') direction = 'desc';
    setSortConfig({ key, direction });
  };

  const getSortIcon = (key) => {
    if (!sortConfig || sortConfig.key !== key)
      return <ArrowUpDown className="ml-2 h-3.5 w-3.5 opacity-50 inline-block" />;
    return sortConfig.direction === 'asc' ? (
      <ArrowUp className="ml-2 h-3.5 w-3.5 inline-block" />
    ) : (
      <ArrowDown className="ml-2 h-3.5 w-3.5 inline-block" />
    );
  };

  const filteredRequests = useMemo(
    () =>
      requests.filter(
        (r) =>
          (r.id || '').toLowerCase().includes(filters.id.toLowerCase()) &&
          (r.participant_slug || '').toLowerCase().includes(filters.slug.toLowerCase()) &&
          (r.vc_type || '').toLowerCase().includes(filters.vcType.toLowerCase()) &&
          getIdentityProofDisplay(r.interact_method)
            .toLowerCase()
            .includes(filters.interactMethod.toLowerCase()) &&
          (r.status || '').toLowerCase().includes(filters.status.toLowerCase()) &&
          (r.created_at || '').toLowerCase().includes(filters.createdAt.toLowerCase()),
      ),
    [requests, filters],
  );

  const sortedRequests = useMemo(() => {
    if (!sortConfig) return filteredRequests;
    return [...filteredRequests].sort((a, b) => {
      const aVal = (a[sortConfig.key] ?? '').toString().toLowerCase();
      const bVal = (b[sortConfig.key] ?? '').toString().toLowerCase();
      if (aVal < bVal) return sortConfig.direction === 'asc' ? -1 : 1;
      if (aVal > bVal) return sortConfig.direction === 'asc' ? 1 : -1;
      return 0;
    });
  }, [filteredRequests, sortConfig]);

  if (loading) {
    return (
      <PageLayout>
        <PageHeader title="Requests" />
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchRequests} />;

  return (
    <PageLayout>
      <PageHeader
        title="Requests"
        badge={
          <Badge size="lg" className="uppercase font-medium">
            {requests.length} total
          </Badge>
        }
      />

      <PageSection title="Credential requests">
        <div className="rounded-md border border-white/10 bg-background-200/5">
          <Table className="text-sm">
            <TableHeader>
              <TableRow className="border-b-white/10 hover:bg-transparent">
                <TableHead
                  onClick={() => handleSort('id')}
                  className="cursor-pointer text-white/80"
                >
                  Request ID {getSortIcon('id')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('participant_slug')}
                  className="cursor-pointer text-white/80"
                >
                  Alias {getSortIcon('participant_slug')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('vc_type')}
                  className="cursor-pointer text-white/80"
                >
                  VC Type {getSortIcon('vc_type')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('interact_method')}
                  className="cursor-pointer text-white/80"
                >
                  Identity Proof {getSortIcon('interact_method')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('status')}
                  className="cursor-pointer text-white/80"
                >
                  Status {getSortIcon('status')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('created_at')}
                  className="cursor-pointer text-white/80"
                >
                  Created at {getSortIcon('created_at')}
                </TableHead>
                <TableHead className="text-white/80">Actions</TableHead>
              </TableRow>
              <TableRow className="bg-background-200/20 hover:bg-background-200/20 border-none">
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.id}
                    onChange={(e) => setFilters((f) => ({ ...f, id: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.slug}
                    onChange={(e) => setFilters((f) => ({ ...f, slug: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.vcType}
                    onChange={(e) => setFilters((f) => ({ ...f, vcType: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.interactMethod}
                    onChange={(e) =>
                      setFilters((f) => ({ ...f, interactMethod: e.target.value }))
                    }
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.status}
                    onChange={(e) => setFilters((f) => ({ ...f, status: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.createdAt}
                    onChange={(e) => setFilters((f) => ({ ...f, createdAt: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead />
              </TableRow>
            </TableHeader>
            <TableBody>
              {sortedRequests.map((r) => (
                <TableRow
                  key={r.id}
                  onClick={() => navigate(`/requests/${r.id}`)}
                  className="cursor-pointer border-b-white/5 hover:bg-white/5 transition-colors"
                >
                  <TableCell>
                    <Badge variant="info">{formatIdentifier(r.id)}</Badge>
                  </TableCell>
                  <TableCell className="capitalize">{r.participant_slug || '—'}</TableCell>
                  <TableCell>
                    <Badge variant="info">{r.vc_type}</Badge>
                  </TableCell>
                  <TableCell className="text-white/80">
                    {getIdentityProofDisplay(r.interact_method)}
                  </TableCell>
                  <TableCell>
                    <Badge variant="status" state={r.status}>
                      {r.status}
                    </Badge>
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    <FormatDate date={r.created_at} />
                  </TableCell>
                  <TableCell>
                    <Button
                      variant="link"
                      onClick={(e) => {
                        e.stopPropagation();
                        navigate(`/requests/${r.id}`);
                      }}
                    >
                      See request
                      <ArrowRight />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {sortedRequests.length === 0 && requests.length > 0 && (
            <div className="p-8 text-center text-muted-foreground italic text-sm">
              No requests match the current filters
            </div>
          )}
        </div>
      </PageSection>
    </PageLayout>
  );
};

export default Requests;
