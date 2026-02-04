import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const Requests = () => {
  const [requests, setRequests] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
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

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div style={{ padding: '20px' }}>
      <h1>Requests</h1>
      <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: '10px' }}>
        <thead>
          <tr style={{ borderBottom: '2px solid #333', textAlign: 'left' }}>
            <th style={{ padding: '10px' }}>ID</th>
            <th style={{ padding: '10px' }}>Slug</th>
            <th style={{ padding: '10px' }}>VC Type</th>
            <th style={{ padding: '10px' }}>Interact Method</th>
            <th style={{ padding: '10px' }}>Status</th>
            <th style={{ padding: '10px' }}>Created At</th>
          </tr>
        </thead>
        <tbody>
          {requests.map((req) => (
            <tr
              key={req.id}
              onClick={() => handleRowClick(req.id)}
              style={{
                borderBottom: '1px solid #ccc',
                cursor: 'pointer',
                ':hover': { backgroundColor: '#f5f5f5' },
              }}
            >
              <td style={{ padding: '10px' }}>{req.id}</td>
              <td style={{ padding: '10px' }}>{req.participant_slug}</td>
              <td style={{ padding: '10px' }}>{req.vc_type}</td>
              <td style={{ padding: '10px' }}>{req.interact_method.join(', ')}</td>
              <td style={{ padding: '10px' }}>{req.status}</td>
              <td style={{ padding: '10px' }}>{req.created_at}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Requests;
