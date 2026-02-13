import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const StatusBadge = ({ status, isVcIssued }) => {
  const getStatusColor = (status, isVcIssued) => {
    switch (status?.toLowerCase()) {
      case 'processing':
      case 'proccesing':
        return { bg: 'rgba(240, 255, 0, 0.15)', color: '#f0ff00', border: '#f0ff00' };
      case 'pending':
        return { bg: 'rgba(255, 165, 0, 0.15)', color: '#ffa500', border: '#ffa500' };
      case 'approved':
        return { bg: 'rgba(0, 240, 255, 0.15)', color: '#00f0ff', border: '#00f0ff' };
      case 'finalized':
        const color = isVcIssued ? '#00ff41' : '#ff0040';
        return { bg: `${color}26`, color: color, border: color }; // 26 is ~15% alpha
      default:
        return { bg: 'rgba(0, 240, 255, 0.15)', color: '#00f0ff', border: '#00f0ff' };
    }
  };

  const colors = getStatusColor(status, isVcIssued);

  return (
    <span
      style={{
        display: 'inline-block',
        padding: '4px 12px',
        borderRadius: '12px',
        fontSize: '0.85em',
        fontWeight: 'bold',
        backgroundColor: colors.bg,
        color: colors.color,
        border: `2px solid ${colors.border}`,
        boxShadow: `0 0 10px ${colors.bg}`,
        textTransform: 'uppercase',
        letterSpacing: '1px',
      }}
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

  const inputStyle = {
    width: '100%',
    padding: '8px',
    backgroundColor: 'rgba(10, 14, 39, 0.8)',
    border: '2px solid #00f0ff',
    borderRadius: '4px',
    color: '#e0e0e0',
    fontSize: '0.9em',
    outline: 'none',
    boxShadow: '0 0 5px rgba(0, 240, 255, 0.3)',
  };

  const getSortIndicator = (key) => {
    if (sortConfig.key !== key) {
      return ' ⇅';
    }
    return sortConfig.direction === 'asc' ? ' ▲' : ' ▼';
  };

  const headerStyle = (key) => ({
    padding: '15px',
    color: '#00f0ff',
    textShadow: '0 0 10px rgba(0, 240, 255, 0.8)',
    cursor: 'pointer',
    userSelect: 'none',
    transition: 'all 0.3s ease',
  });

  if (loading) return <div style={{ padding: '20px', color: '#00f0ff' }}>Loading...</div>;
  if (error) return <div style={{ padding: '20px', color: '#ff0040' }}>Error: {error}</div>;

  return (
    <div style={{ padding: '30px', width: '100%', minHeight: '100vh' }}>
      <h1>Requests</h1>
      <table
        style={{
          width: '100%',
          borderCollapse: 'collapse',
          marginTop: '10px',
          backgroundColor: 'rgba(26, 29, 53, 0.5)',
          border: '2px solid #00f0ff',
          boxShadow: '0 0 20px rgba(0, 240, 255, 0.3)',
        }}
      >
        <thead>
          <tr
            style={{
              borderBottom: '2px solid #00f0ff',
              textAlign: 'left',
              backgroundColor: 'rgba(0, 240, 255, 0.1)',
            }}
          >
            <th
              style={headerStyle('id')}
              onClick={() => handleSort('id')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              ID{getSortIndicator('id')}
            </th>
            <th
              style={headerStyle('slug')}
              onClick={() => handleSort('slug')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Slug{getSortIndicator('slug')}
            </th>
            <th
              style={headerStyle('vcType')}
              onClick={() => handleSort('vcType')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              VC Type{getSortIndicator('vcType')}
            </th>
            <th
              style={headerStyle('interactMethod')}
              onClick={() => handleSort('interactMethod')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Interact Method{getSortIndicator('interactMethod')}
            </th>
            <th
              style={headerStyle('status')}
              onClick={() => handleSort('status')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Status{getSortIndicator('status')}
            </th>
            <th
              style={headerStyle('createdAt')}
              onClick={() => handleSort('createdAt')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Created At{getSortIndicator('createdAt')}
            </th>
          </tr>
          <tr style={{ backgroundColor: 'rgba(10, 14, 39, 0.6)' }}>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter ID..."
                value={filters.id}
                onChange={(e) => handleFilterChange('id', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter Slug..."
                value={filters.slug}
                onChange={(e) => handleFilterChange('slug', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter VC Type..."
                value={filters.vcType}
                onChange={(e) => handleFilterChange('vcType', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter Method..."
                value={filters.interactMethod}
                onChange={(e) => handleFilterChange('interactMethod', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter Status..."
                value={filters.status}
                onChange={(e) => handleFilterChange('status', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter Date..."
                value={filters.createdAt}
                onChange={(e) => handleFilterChange('createdAt', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
          </tr>
        </thead>
        <tbody>
          {sortedRequests.map((req) => (
            <tr
              key={req.id}
              onClick={() => handleRowClick(req.id)}
              style={{
                borderBottom: '1px solid rgba(0, 240, 255, 0.3)',
                cursor: 'pointer',
                transition: 'all 0.3s ease',
                backgroundColor: 'transparent',
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.1)';
                e.currentTarget.style.boxShadow = '0 0 15px rgba(0, 240, 255, 0.5)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
                e.currentTarget.style.boxShadow = 'none';
              }}
            >
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{req.id}</td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{req.participant_slug}</td>
              <td style={{ padding: '12px', color: '#ff00ff' }}>{req.vc_type}</td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>
                {req.interact_method.join(', ')}
              </td>
              <td style={{ padding: '12px' }}>
                <StatusBadge status={req.status} isVcIssued={req.is_vc_issued} />
              </td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{req.created_at}</td>
            </tr>
          ))}
        </tbody>
      </table>
      {sortedRequests.length === 0 && requests.length > 0 && (
        <div
          style={{
            textAlign: 'center',
            padding: '20px',
            color: '#ff00ff',
            fontSize: '1.1em',
          }}
        >
          No requests match the current filters
        </div>
      )}
    </div>
  );
};

export default Requests;
