import { Link, Outlet } from 'react-router-dom';

const Layout = () => {
  return (
    <div>
      <nav style={{ padding: '20px', borderBottom: '1px solid #ccc', marginBottom: '20px' }}>
        <ul style={{ display: 'flex', gap: '20px', listStyle: 'none', margin: 0, padding: 0 }}>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/about">About</Link>
          </li>
          <li>
            <Link to="/dashboard">Dashboard</Link>
          </li>
          <li>
            <Link to="/minions">Minions</Link>
          </li>
          <li>
            <Link to="/requests">Requests</Link>
          </li>
        </ul>
      </nav>
      <main>
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;
