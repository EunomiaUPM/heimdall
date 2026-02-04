import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import BooleanBadge from '../components/BooleanBadge';

const TruncatedId = ({ id }) => {
  const [showTooltip, setShowTooltip] = useState(false);
  const [copied, setCopied] = useState(false);

  const shouldTruncate = id.length > 20;
  const displayId = shouldTruncate ? `${id.substring(0, 17)}...` : id;

  const handleCopy = (e) => {
    e.stopPropagation();
    navigator.clipboard.writeText(id);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div
      style={{ position: 'relative', display: 'inline-block' }}
      onMouseEnter={() => setShowTooltip(true)}
      onMouseLeave={() => setShowTooltip(false)}
    >
      <span>{displayId}</span>
      {showTooltip && (
        <div
          style={{
            position: 'absolute',
            top: '100%',
            left: '0',
            zIndex: 1000,
            backgroundColor: '#1a1d35',
            border: '2px solid #00f0ff',
            padding: '10px',
            borderRadius: '4px',
            boxShadow: '0 0 20px rgba(0, 240, 255, 0.5)',
            minWidth: '600px',
            maxWidth: '900px',
            wordBreak: 'break-all',
            whiteSpace: 'normal',
            color: '#e0e0e0',
          }}
        >
          <div style={{ marginBottom: '8px' }}>{id}</div>
          <button
            onClick={handleCopy}
            style={{
              padding: '4px 8px',
              cursor: 'pointer',
              fontSize: '12px',
              fontWeight: 'bold',
              color: '#00f0ff',
              backgroundColor: 'rgba(0, 240, 255, 0.1)',
              border: '2px solid #00f0ff',
              borderRadius: '4px',
            }}
          >
            {copied ? 'Copied!' : 'Copy'}
          </button>
        </div>
      )}
    </div>
  );
};

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
  const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });
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

  const formatDate = (dateString) => {
    if (!dateString) return '-';
    return new Date(dateString).toLocaleString();
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

  const getSortValue = (minion, key) => {
    switch (key) {
      case 'id':
        return minion.participant_id;
      case 'slug':
        return minion.participant_slug;
      case 'type':
        return minion.participant_type;
      case 'issuedVC':
        return minion.is_vc_issued ? 'Yes' : 'No';
      case 'savedAt':
        return minion.saved_at;
      case 'isMe':
        return minion.is_me ? 'Yes' : 'No';
      default:
        return '';
    }
  };

  const filteredMinions = minions.filter((minion) => {
    const issuedVCText = minion.is_vc_issued ? 'yes' : 'no';
    const isMeText = minion.is_me ? 'yes' : 'no';

    return (
      minion.participant_id.toLowerCase().includes(filters.id.toLowerCase()) &&
      minion.participant_slug.toLowerCase().includes(filters.slug.toLowerCase()) &&
      minion.participant_type.toLowerCase().includes(filters.type.toLowerCase()) &&
      issuedVCText.includes(filters.issuedVC.toLowerCase()) &&
      minion.saved_at.toLowerCase().includes(filters.savedAt.toLowerCase()) &&
      isMeText.includes(filters.isMe.toLowerCase())
    );
  });

  const sortedMinions = [...filteredMinions].sort((a, b) => {
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

  const headerStyle = {
    padding: '15px',
    color: '#00f0ff',
    textShadow: '0 0 10px rgba(0, 240, 255, 0.8)',
    cursor: 'pointer',
    userSelect: 'none',
    transition: 'all 0.3s ease',
  };

  if (loading) return <div style={{ padding: '20px', color: '#00f0ff' }}>Loading...</div>;
  if (error) return <div style={{ padding: '20px', color: '#ff0040' }}>Error: {error}</div>;

  return (
    <div style={{ padding: '30px', width: '100%', minHeight: '100vh' }}>
      <h1>Minions List</h1>
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
              style={headerStyle}
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
              style={headerStyle}
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
              style={headerStyle}
              onClick={() => handleSort('type')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Type{getSortIndicator('type')}
            </th>
            <th
              style={headerStyle}
              onClick={() => handleSort('issuedVC')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Issued VC{getSortIndicator('issuedVC')}
            </th>
            <th
              style={headerStyle}
              onClick={() => handleSort('savedAt')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Saved At{getSortIndicator('savedAt')}
            </th>
            <th
              style={headerStyle}
              onClick={() => handleSort('isMe')}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = 'rgba(0, 240, 255, 0.2)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = 'transparent';
              }}
            >
              Is Me{getSortIndicator('isMe')}
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
                placeholder="Filter Type..."
                value={filters.type}
                onChange={(e) => handleFilterChange('type', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter..."
                value={filters.issuedVC}
                onChange={(e) => handleFilterChange('issuedVC', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter Date..."
                value={filters.savedAt}
                onChange={(e) => handleFilterChange('savedAt', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
            <th style={{ padding: '10px' }}>
              <input
                type="text"
                placeholder="Filter..."
                value={filters.isMe}
                onChange={(e) => handleFilterChange('isMe', e.target.value)}
                style={inputStyle}
                onClick={(e) => e.stopPropagation()}
              />
            </th>
          </tr>
        </thead>
        <tbody>
          {sortedMinions.map((minion) => (
            <tr
              key={minion.participant_id}
              onClick={() => handleRowClick(minion.participant_id)}
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
              <td style={{ padding: '12px', color: '#e0e0e0' }}>
                <TruncatedId id={minion.participant_id} />
              </td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{minion.participant_slug}</td>
              <td style={{ padding: '12px', color: '#ff00ff' }}>{minion.participant_type}</td>
              <td style={{ padding: '12px' }}>
                <BooleanBadge value={minion.is_vc_issued} />
              </td>
              <td style={{ padding: '12px', color: '#e0e0e0' }}>{formatDate(minion.saved_at)}</td>
              <td style={{ padding: '12px' }}>
                <BooleanBadge value={minion.is_me} />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      {sortedMinions.length === 0 && minions.length > 0 && (
        <div
          style={{
            textAlign: 'center',
            padding: '20px',
            color: '#ff00ff',
            fontSize: '1.1em',
          }}
        >
          No minions match the current filters
        </div>
      )}
    </div>
  );
};

export default Minions;
