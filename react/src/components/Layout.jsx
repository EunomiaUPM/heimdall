import { Link, Outlet, useLocation } from 'react-router-dom';

const Layout = () => {
  const location = useLocation();

  const isActive = (path) => {
    return location.pathname === path || (path !== '/' && location.pathname.startsWith(path));
  };

  const getLinkStyle = (path) => ({
    display: 'block',
    padding: '12px 16px',
    textDecoration: 'none',
    color: isActive(path) ? '#00f0ff' : '#e0e0e0',
    backgroundColor: isActive(path) ? 'rgba(0, 240, 255, 0.15)' : 'transparent',
    borderLeft: isActive(path) ? '4px solid #00f0ff' : '4px solid transparent',
    borderRadius: '4px',
    transition: 'all 0.3s ease',
    fontWeight: isActive(path) ? 'bold' : 'normal',
    textShadow: isActive(path) ? '0 0 10px rgba(0, 240, 255, 0.8)' : 'none',
  });

  return (
    <div
      style={{
        display: 'flex',
        minHeight: '100vh',
        background: 'linear-gradient(135deg, #0a0e27 0%, #1a1d35 100%)',
      }}
    >
      <nav
        style={{
          width: '150px',
          backgroundColor: '#0a0e27',
          borderRight: '2px solid #00f0ff',
          boxShadow: '2px 0 20px rgba(0, 240, 255, 0.3)',
          padding: '30px 0',
          position: 'fixed',
          height: '100vh',
          left: 0,
          top: 0,
          overflowY: 'auto',
        }}
      >
        <div
          style={{
            padding: '0 16px 20px',
            borderBottom: '2px solid #00f0ff',
            marginBottom: '20px',
            boxShadow: '0 2px 10px rgba(0, 240, 255, 0.3)',
          }}
        >
          <h2
            style={{
              margin: 0,
              fontSize: '1.5em',
              color: '#00f0ff',
              textShadow: '0 0 15px rgba(0, 240, 255, 0.8)',
              fontFamily: 'Courier New, monospace',
            }}
          >
            HEIMDALL
          </h2>
        </div>
        <ul
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: '8px',
            listStyle: 'none',
            margin: 0,
            padding: '0 12px',
          }}
        >
          <li>
            <Link to="/" style={getLinkStyle('/')}>
              Home
            </Link>
          </li>
          <li>
            <Link to="/about" style={getLinkStyle('/about')}>
              About
            </Link>
          </li>
          <li>
            <Link to="/minions" style={getLinkStyle('/minions')}>
              Minions
            </Link>
          </li>
          <li>
            <Link to="/requests" style={getLinkStyle('/requests')}>
              Requests
            </Link>
          </li>
          {import.meta.env.VITE_WALLET_ACTIVE === 'true' && (
            <li>
              <Link to="/wallet" style={getLinkStyle('/wallet')}>
                Wallet
              </Link>
            </li>
          )}
        </ul>
      </nav>
      <main
        style={{
          marginLeft: '150px',
          width: 'calc(100% - 150px)',
          padding: '0',
          minHeight: '100vh',
        }}
      >
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;
