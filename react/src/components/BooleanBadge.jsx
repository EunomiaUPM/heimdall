const BooleanBadge = ({ value }) => {
  const isTrue = value === true || value === 'true' || value === 'Yes';

  return (
    <span
      style={{
        display: 'inline-block',
        padding: '4px 12px',
        borderRadius: '12px',
        fontSize: '0.85em',
        fontWeight: 'bold',
        backgroundColor: isTrue ? 'rgba(0, 255, 65, 0.15)' : 'rgba(255, 0, 64, 0.15)',
        color: isTrue ? '#00ff41' : '#ff0040',
        border: `2px solid ${isTrue ? '#00ff41' : '#ff0040'}`,
        boxShadow: `0 0 10px ${isTrue ? 'rgba(0, 255, 65, 0.3)' : 'rgba(255, 0, 64, 0.3)'}`,
        textTransform: 'uppercase',
        letterSpacing: '1px',
      }}
    >
      {isTrue ? 'Yes' : 'No'}
    </span>
  );
};

export default BooleanBadge;
