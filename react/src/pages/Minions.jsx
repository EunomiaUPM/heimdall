import { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { ArrowRight, ArrowUpDown, ArrowUp, ArrowDown } from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { cn, formatIdentifier } from '@/lib/utils';
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
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';
import { FormatDate } from '@/components/ui/format-date';

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
  const [sortConfig, setSortConfig] = useState(null);
  const navigate = useNavigate();

  const fetchMinions = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/minions/all`);
      if (!response.ok) throw new Error('Failed to fetch minions');
      const data = await response.json();
      setMinions(data);
    } catch (err) {
      console.error('Error fetching minions:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMinions();
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

  const filteredMinions = useMemo(
    () =>
      minions.filter((m) => {
        const issuedVCText = m.is_vc_issued ? 'issued' : 'pending';
        const isMeText = m.is_me ? 'yes' : 'no';
        return (
          (m.participant_id || '').toLowerCase().includes(filters.id.toLowerCase()) &&
          (m.participant_slug || '').toLowerCase().includes(filters.slug.toLowerCase()) &&
          (m.participant_type || '').toLowerCase().includes(filters.type.toLowerCase()) &&
          issuedVCText.includes(filters.issuedVC.toLowerCase()) &&
          (m.saved_at || '').toLowerCase().includes(filters.savedAt.toLowerCase()) &&
          isMeText.includes(filters.isMe.toLowerCase())
        );
      }),
    [minions, filters],
  );

  const sortedMinions = useMemo(() => {
    if (!sortConfig) return filteredMinions;
    return [...filteredMinions].sort((a, b) => {
      const aVal = (a[sortConfig.key] ?? '').toString().toLowerCase();
      const bVal = (b[sortConfig.key] ?? '').toString().toLowerCase();
      if (aVal < bVal) return sortConfig.direction === 'asc' ? -1 : 1;
      if (aVal > bVal) return sortConfig.direction === 'asc' ? 1 : -1;
      return 0;
    });
  }, [filteredMinions, sortConfig]);

  const myAgent = minions.find((m) => m.is_me);

  if (loading) {
    return (
      <PageLayout>
        <PageHeader title="Participants" />
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          {[1, 2, 3].map((i) => (
            <Skeleton key={i} className="h-32 w-full rounded-xl" />
          ))}
        </div>
        <Skeleton className="h-64 w-full rounded-xl" />
      </PageLayout>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchMinions} />;

  return (
    <PageLayout>
      <PageHeader
        title="Participants"
        badge={
          <Badge size="lg" className="uppercase font-medium">
            {minions.length} total
          </Badge>
        }
      />

      {myAgent && (
        <div className="mb-8">
          <Card className="bg-background-200/15 overflow-hidden relative">
            <CardHeader className="pb-2">
              <div className="flex justify-between items-center">
                <div>
                  <p className="text-xs font-semibold text-brand-sky uppercase tracking-wider mb-1">
                    My Local Agent
                  </p>
                  <CardTitle className="text-2xl">
                    {myAgent.participant_slug || 'Unnamed Agent'}
                  </CardTitle>
                </div>
                <Badge variant="status" state="active">
                  Active
                </Badge>
              </div>
            </CardHeader>
            <CardContent>
              <div className="space-y-4 mt-2">
                <div className="space-y-1">
                  <p className="text-[10px] text-muted-foreground uppercase">DID Identifier</p>
                  <div className="font-mono text-xs break-all bg-background-200 p-2 rounded border border-white/10">
                    {myAgent.participant_id}
                  </div>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 items-end">
                  <div className="space-y-1">
                    <p className="text-[10px] text-muted-foreground uppercase">Role</p>
                    <div className="flex pt-1">
                      <Badge variant="role" dsrole={myAgent.participant_type}>
                        {myAgent.participant_type}
                      </Badge>
                    </div>
                  </div>
                  <div className="flex items-end justify-end">
                    <Button
                      variant="link"
                      onClick={() => navigate(`/participants/${myAgent.participant_id}`)}
                    >
                      View My Profile
                      <ArrowRight />
                    </Button>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      <PageSection title="Network Participants">
        <div className="rounded-md border border-white/10 bg-background-200/5">
          <Table className="text-sm">
            <TableHeader>
              <TableRow className="border-b-white/10 hover:bg-transparent">
                <TableHead
                  onClick={() => handleSort('participant_id')}
                  className="cursor-pointer text-white/80"
                >
                  Participant DID {getSortIcon('participant_id')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('participant_slug')}
                  className="cursor-pointer text-white/80"
                >
                  Alias {getSortIcon('participant_slug')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('participant_type')}
                  className="cursor-pointer text-white/80"
                >
                  Role {getSortIcon('participant_type')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('is_vc_issued')}
                  className="cursor-pointer text-white/80"
                >
                  Verifiable Credential {getSortIcon('is_vc_issued')}
                </TableHead>
                <TableHead
                  onClick={() => handleSort('saved_at')}
                  className="cursor-pointer text-white/80"
                >
                  Added on {getSortIcon('saved_at')}
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
                    value={filters.type}
                    onChange={(e) => setFilters((f) => ({ ...f, type: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.issuedVC}
                    onChange={(e) => setFilters((f) => ({ ...f, issuedVC: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead className="p-2">
                  <Input
                    placeholder="Filter…"
                    value={filters.savedAt}
                    onChange={(e) => setFilters((f) => ({ ...f, savedAt: e.target.value }))}
                    onClick={(e) => e.stopPropagation()}
                    className="h-8"
                  />
                </TableHead>
                <TableHead />
              </TableRow>
            </TableHeader>
            <TableBody>
              {sortedMinions.map((m) => (
                <TableRow
                  key={m.participant_id}
                  onClick={() => navigate(`/participants/${m.participant_id}`)}
                  className="cursor-pointer border-b-white/5 hover:bg-white/5 transition-colors"
                >
                  <TableCell>
                    <Badge variant="info">{formatIdentifier(m.participant_id)}</Badge>
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center gap-3">
                      <div
                        className={cn(
                          'w-8 h-8 rounded-lg flex items-center justify-center font-bold text-xs',
                          m.is_me
                            ? 'bg-brand-sky text-white'
                            : 'bg-background-200 text-muted-foreground',
                        )}
                      >
                        {(m.participant_slug || 'U').charAt(0).toUpperCase()}
                      </div>
                      <div className="flex items-baseline gap-2">
                        <span className="font-medium capitalize">
                          {m.participant_slug || 'Unknown'}
                        </span>
                        {m.is_me && <Badge size="sm">IT'S ME</Badge>}
                      </div>
                    </div>
                  </TableCell>
                  <TableCell>
                    <Badge variant="role" dsrole={m.participant_type}>
                      {m.participant_type}
                    </Badge>
                  </TableCell>
                  <TableCell>
                    {m.is_me ? (
                      ''
                    ) : (
                      <span
                        className={cn(
                          'font-medium',
                          m.is_vc_issued ? 'text-success-400' : 'text-warn-400',
                        )}
                      >
                        {m.is_vc_issued ? 'Issued' : 'Pending'}
                      </span>
                    )}
                  </TableCell>
                  <TableCell className="text-muted-foreground">
                    <FormatDate date={m.saved_at} />
                  </TableCell>
                  <TableCell>
                    <Button
                      variant="link"
                      onClick={(e) => {
                        e.stopPropagation();
                        navigate(`/participants/${m.participant_id}`);
                      }}
                    >
                      See agent
                      <ArrowRight />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {sortedMinions.length === 0 && minions.length > 0 && (
            <div className="p-8 text-center text-muted-foreground italic text-sm">
              No participants match the current filters
            </div>
          )}
        </div>
      </PageSection>
    </PageLayout>
  );
};

export default Minions;
