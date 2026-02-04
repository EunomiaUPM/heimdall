import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import BooleanBadge from '../components/BooleanBadge';

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

  if (loading) return <div style={{ padding: '20px', color: '#00f0ff' }}>Loading...</div>;
  if (error) return <div style={{ padding: '20px', color: '#ff0040' }}>Error: {error}</div>;
  if (!minion) return <div style={{ padding: '20px', color: '#ff0040' }}>Minion not found</div>;

  return (
    <div style={{ padding: '30px', width: '100%', minHeight: '100vh' }}>
      <div style={{ position: 'relative', marginBottom: '20px' }}>
        <button
          onClick={() => navigate('/minions')}
          style={{
            position: 'absolute',
            left: 0,
            top: '50%',
            transform: 'translateY(-50%)',
            cursor: 'pointer',
            backgroundColor: 'rgba(189, 0, 255, 0.2)',
            border: '2px solid #bd00ff',
            color: '#bd00ff',
            padding: '8px 16px',
            boxShadow: '0 0 15px rgba(189, 0, 255, 0.4)',
            borderRadius: '4px',
            fontSize: '1em',
            transition: 'all 0.3s ease',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.3)';
            e.currentTarget.style.boxShadow = '0 0 20px rgba(189, 0, 255, 0.6)';
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = 'rgba(189, 0, 255, 0.2)';
            e.currentTarget.style.boxShadow = '0 0 15px rgba(189, 0, 255, 0.4)';
          }}
        >
          &larr; Back to List
        </button>
        <h1 style={{ margin: 0, textAlign: 'center' }}>Minion Details</h1>
      </div>
      <div
        style={{
          border: '2px solid #00f0ff',
          padding: '25px',
          borderRadius: '8px',
          marginBottom: '20px',
          textAlign: 'left',
          backgroundColor: 'rgba(26, 29, 53, 0.6)',
          boxShadow: '0 0 20px rgba(0, 240, 255, 0.3)',
        }}
      >
        <p>
          <strong style={{ color: '#00f0ff' }}>Participant ID:</strong>{' '}
          <span
            style={{
              color: '#e0e0e0',
              wordBreak: 'break-word',
              overflowWrap: 'break-word',
              display: 'inline-block',
              maxWidth: '100%',
            }}
          >
            {minion.participant_id}
          </span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Slug:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{minion.participant_slug}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Type:</strong>{' '}
          <span style={{ color: '#ff00ff' }}>{minion.participant_type}</span>
        </p>
        {minion.base_url && (
          <p>
            <strong style={{ color: '#00f0ff' }}>Base URL:</strong>{' '}
            <span style={{ color: '#e0e0e0' }}>{minion.base_url}</span>
          </p>
        )}
        {minion.vc_uri && (
          <p>
            <strong style={{ color: '#00f0ff' }}>VC URI:</strong>{' '}
            <span
              style={{
                color: '#e0e0e0',
                wordBreak: 'break-word',
                overflowWrap: 'break-word',
                display: 'inline-block',
                maxWidth: '100%',
              }}
            >
              {minion.vc_uri}
            </span>
          </p>
        )}
        <p>
          <strong style={{ color: '#00f0ff' }}>VC Issued:</strong>{' '}
          <BooleanBadge value={minion.is_vc_issued} />
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Saved At:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{minion.saved_at}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Last Interaction:</strong>{' '}
          <span style={{ color: '#e0e0e0' }}>{minion.last_interaction}</span>
        </p>
        <p>
          <strong style={{ color: '#00f0ff' }}>Is Me:</strong> <BooleanBadge value={minion.is_me} />
        </p>
      </div>
    </div>
  );
};

export default MinionDetails;
