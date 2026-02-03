import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const Minions = () => {
  const [minions, setMinions] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const navigate = useNavigate();

  const apiUrl = import.meta.env.VITE_API_SERVER_URL;

  useEffect(() => {
    const fetchMinions = async () => {
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
        setError(err.message);
        setLoading(false);
      }
    };

    fetchMinions();
  }, [apiUrl]);

  const handleRowClick = (id) => {
    navigate(`/minions/${id}`);
  };

  const handleFindMe = () => {
    const myMinion = minions.find((m) => m.is_me);
    if (myMinion) {
      navigate(`/minions/${myMinion.participant_id}`);
    } else {
      alert("No minion found with 'is_me' flag.");
    }
  };

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div style={{ padding: '20px' }}>
      <h1>Minions List</h1>
      <button
        onClick={handleFindMe}
        style={{ marginBottom: '20px', padding: '10px 20px', cursor: 'pointer' }}
      >
        Find Me
      </button>
      <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: '10px' }}>
        <thead>
          <tr style={{ borderBottom: '2px solid #333', textAlign: 'left' }}>
            <th style={{ padding: '10px' }}>ID</th>
            <th style={{ padding: '10px' }}>Slug</th>
            <th style={{ padding: '10px' }}>Issued VC</th>
            <th style={{ padding: '10px' }}>Is Me</th>
          </tr>
        </thead>
        <tbody>
          {minions.map((minion) => (
            <tr
              key={minion.participant_id}
              onClick={() => handleRowClick(minion.participant_id)}
              style={{
                borderBottom: '1px solid #ccc',
                cursor: 'pointer',
                ':hover': { backgroundColor: '#f5f5f5' },
              }}
            >
              <td style={{ padding: '10px' }}>{minion.participant_id}</td>
              <td style={{ padding: '10px' }}>{minion.participant_slug}</td>
              <td style={{ padding: '10px' }}>{minion.is_vc_issued ? 'Yes' : 'No'}</td>
              <td style={{ padding: '10px' }}>{minion.is_me ? 'Yes' : 'No'}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Minions;
