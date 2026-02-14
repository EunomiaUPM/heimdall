import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import BooleanBadge from '../components/BooleanBadge';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { ArrowLeft } from 'lucide-react';

const MinionDetails = () => {
  const { id } = useParams();
  const [minion, setMinion] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchMinion = async () => {
      try {
        const response = await fetch(`${apiUrl}/minions/${id}`);
        if (!response.ok) {
          throw new Error('Failed to fetch minion details');
        }
        const data = await response.json();
        setMinion(data);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching minion details:', err);
        setError(err.message);
        setLoading(false);
      }
    };

    fetchMinion();
  }, [id, apiUrl]);

  if (loading) return <div className="p-8 text-brand-sky">Loading...</div>;
  if (error) return <div className="p-8 text-danger">Error: {error}</div>;
  if (!minion) return <div className="p-8 text-danger">Minion not found</div>;

  return (
    <div className="w-full">
      <div className="relative mb-6 flex items-center justify-center">
        <Button
          variant="outline"
          onClick={() => navigate('/minions')}
          className="absolute left-0 border-brand-purple text-brand-purple hover:bg-brand-purple/10 hover:text-brand-purple"
        >
          <ArrowLeft className="mr-2 h-4 w-4" /> Back to List
        </Button>
        <h1 className="text-3xl font-bold text-brand-sky font-ubuntu">Minion Details</h1>
      </div>

      <div className="rounded-lg border border-brand-sky bg-background/60 p-6 shadow-lg shadow-brand-sky/20 text-left">
        <div className="space-y-4">
          <p>
            <strong className="text-brand-sky">Participant ID:</strong>{' '}
            <span className="text-muted-foreground break-all inline-block max-w-full">
              {minion.participant_id}
            </span>
          </p>
          <p>
            <strong className="text-brand-sky">Slug:</strong>{' '}
            <span className="text-muted-foreground">{minion.participant_slug}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Type:</strong>{' '}
            <span className="text-brand-purple">{minion.participant_type}</span>
          </p>
          {minion.base_url && (
            <p>
              <strong className="text-brand-sky">Base URL:</strong>{' '}
              <span className="text-muted-foreground">{minion.base_url}</span>
            </p>
          )}
          {minion.vc_uri && (
            <p>
              <strong className="text-brand-sky">VC URI:</strong>{' '}
              <span className="text-muted-foreground break-all inline-block max-w-full">
                {minion.vc_uri}
              </span>
            </p>
          )}
          <p>
            <strong className="text-brand-sky">VC Issued:</strong>{' '}
            <BooleanBadge value={minion.is_vc_issued} />
          </p>
          <p>
            <strong className="text-brand-sky">Saved At:</strong>{' '}
            <span className="text-muted-foreground">{minion.saved_at}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Last Interaction:</strong>{' '}
            <span className="text-muted-foreground">{minion.last_interaction}</span>
          </p>
          <p>
            <strong className="text-brand-sky">Is Me:</strong> <BooleanBadge value={minion.is_me} />
          </p>
        </div>
      </div>
    </div>
  );
};

export default MinionDetails;
