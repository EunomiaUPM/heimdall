import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';

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

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!minion) return <div>Minion not found</div>;

  return (
    <div style={{ padding: '20px' }}>
      <button
        onClick={() => navigate('/minions')}
        style={{ marginBottom: '20px', cursor: 'pointer' }}
      >
        &larr; Back to List
      </button>
      <h1>Minion Details</h1>
      <div style={{ border: '1px solid #ccc', padding: '20px', borderRadius: '8px' }}>
        <p>
          <strong>ID:</strong> {minion.participant_id}
        </p>
        <p>
          <strong>Slug:</strong> {minion.participant_slug}
        </p>
        <p>
          <strong>Type:</strong> {minion.participant_type}
        </p>
        <p>
          <strong>Base URL:</strong> {minion.base_url || 'N/A'}
        </p>
        <p>
          <strong>VC URI:</strong> {minion.vc_uri || 'N/A'}
        </p>
        <p>
          <strong>VC Issued:</strong> {minion.is_vc_issued ? 'Yes' : 'No'}
        </p>
        <p>
          <strong>Saved At:</strong> {minion.saved_at}
        </p>
        <p>
          <strong>Last Interaction:</strong> {minion.last_interaction}
        </p>
        <p>
          <strong>Is Me:</strong> {minion.is_me ? 'Yes' : 'No'}
        </p>
      </div>
    </div>
  );
};

export default MinionDetails;
