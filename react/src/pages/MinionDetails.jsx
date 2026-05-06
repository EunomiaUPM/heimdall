import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  ArrowLeft,
  Shield,
  Globe,
  Cpu,
  Key,
  Calendar,
  Activity,
} from 'lucide-react';
import { VITE_API_SERVER_URL as apiUrl } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { InfoList } from '@/components/ui/info-list';
import { GeneralErrorComponent } from '@/components/GeneralErrorComponent';

const MinionDetails = () => {
  const { id } = useParams();
  const [minion, setMinion] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  const fetchMinion = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch(`${apiUrl}/minions/${id}`);
      if (!response.ok) throw new Error('Failed to fetch minion details');
      const data = await response.json();
      setMinion(data);
    } catch (err) {
      console.error('Error fetching minion details:', err);
      setError(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMinion();
  }, [id]);

  if (loading) {
    return (
      <PageLayout>
        <PageHeader title="Participant Details" />
        <div className="space-y-6">
          <Skeleton className="h-48 w-full rounded-xl" />
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Skeleton className="h-64 w-full rounded-xl" />
            <Skeleton className="h-64 w-full rounded-xl" />
          </div>
        </div>
      </PageLayout>
    );
  }

  if (error) return <GeneralErrorComponent error={error} reset={fetchMinion} />;

  if (!minion) {
    return (
      <PageLayout>
        <PageHeader title="Participant Details" />
        <p className="text-muted-foreground italic">Participant not found.</p>
      </PageLayout>
    );
  }

  const formatDate = (d) => (d ? new Date(d).toLocaleString() : 'N/A');

  return (
    <PageLayout>
      <PageHeader
        title={minion.participant_slug || 'Participant Details'}
        badge={
          <div className="flex gap-2">
            <Badge variant="role" dsrole={minion.participant_type}>
              {minion.participant_type}
            </Badge>
            {minion.is_me && <Badge variant="info">Local Agent</Badge>}
          </div>
        }
      >
        <Button
          variant="link"
          className="mt-2 px-0"
          onClick={() => navigate('/participants')}
        >
          <ArrowLeft className="mr-1 h-4 w-4" /> Back to participants
        </Button>
      </PageHeader>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Left column: Identity info */}
        <div className="lg:col-span-2 space-y-6">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-lg font-semibold flex items-center gap-2">
                <Shield className="h-5 w-5 text-brand-sky" />
                Identity Information
              </CardTitle>
            </CardHeader>
            <CardContent>
              <InfoList
                items={[
                  {
                    label: 'Identifier (DID)',
                    value: {
                      type: 'custom',
                      content: (
                        <div className="font-mono text-xs break-all bg-background-200 p-2 rounded border border-white/10">
                          {minion.participant_id}
                        </div>
                      ),
                    },
                  },
                  minion.base_url
                    ? {
                        label: 'Base URL',
                        value: {
                          type: 'custom',
                          content: (
                            <a
                              href={minion.base_url}
                              target="_blank"
                              rel="noreferrer"
                              className="flex items-center gap-1 text-brand-sky hover:underline"
                            >
                              <Globe className="h-3 w-3" />
                              {minion.base_url}
                            </a>
                          ),
                        },
                      }
                    : null,
                  minion.vc_uri
                    ? {
                        label: 'VC URI',
                        value: {
                          type: 'custom',
                          content: (
                            <div className="font-mono text-xs break-all bg-background-200 p-2 rounded border border-white/10">
                              {minion.vc_uri}
                            </div>
                          ),
                        },
                      }
                    : null,
                  {
                    label: 'Verifiable Credential',
                    value: {
                      type: 'custom',
                      content: (
                        <span
                          className={
                            minion.is_vc_issued
                              ? 'font-medium text-success-400'
                              : 'font-medium text-warn-400'
                          }
                        >
                          {minion.is_vc_issued ? 'Issued' : 'Pending'}
                        </span>
                      ),
                    },
                  },
                ].filter(Boolean)}
              />
            </CardContent>
          </Card>
        </div>

        {/* Right column: Activity */}
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-base font-semibold flex items-center gap-2">
                <Calendar className="h-4 w-4 text-brand-sky" />
                Timestamps
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between items-center text-sm">
                <span className="text-muted-foreground flex items-center gap-2">
                  <Key className="h-3 w-3" /> First Registered
                </span>
                <span className="font-medium">{formatDate(minion.saved_at)}</span>
              </div>
              <Separator />
              <div className="flex justify-between items-center text-sm">
                <span className="text-muted-foreground flex items-center gap-2">
                  <Activity className="h-3 w-3" /> Last Interaction
                </span>
                <span className="font-medium">
                  {minion.last_interaction ? formatDate(minion.last_interaction) : 'None'}
                </span>
              </div>
            </CardContent>
          </Card>

          {minion.extra_fields && Object.keys(minion.extra_fields).length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base font-semibold flex items-center gap-2">
                  <Cpu className="h-4 w-4 text-brand-sky" />
                  Extended Metadata
                </CardTitle>
              </CardHeader>
              <CardContent>
                <pre className="text-[10px] bg-background-300 p-3 rounded-lg overflow-x-auto max-h-[300px]">
                  {JSON.stringify(minion.extra_fields, null, 2)}
                </pre>
              </CardContent>
            </Card>
          )}
        </div>
      </div>
    </PageLayout>
  );
};

export default MinionDetails;
