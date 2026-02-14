import { Link, Outlet, useLocation } from 'react-router-dom';
import { cn } from '@/lib/utils';

const Layout = () => {
  const location = useLocation();

  const isActive = (path) => {
    return location.pathname === path || (path !== '/' && location.pathname.startsWith(path));
  };

  const navItems = [
    { name: 'Home', path: '/' },
    { name: 'About', path: '/about' },
    { name: 'Minions', path: '/minions' },
    { name: 'Requests', path: '/requests' },
  ];

  if (import.meta.env.VITE_WALLET_ACTIVE === 'true') {
    navItems.push({ name: 'Wallet', path: '/wallet' });
  }

  return (
    <div className="flex min-h-screen bg-background font-sans text-brand-sky antialiased selection:bg-brand-sky selection:text-brand-black">
      {/* Sidebar */}
      <aside className="fixed left-0 top-0 z-40 h-screen w-64 border-r border-stroke bg-background-200">
        <div className="flex h-16 items-center border-b border-stroke px-6">
          <h2 className="text-xl font-bold tracking-tight text-brand-sky">HEIMDALL</h2>
        </div>
        <nav className="space-y-1 p-4">
          {navItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={cn(
                'flex items-center rounded-md px-3 py-2 text-sm font-medium transition-colors mb-1',
                isActive(item.path)
                  ? 'bg-brand-sky/10 text-brand-sky border-l-4 border-brand-sky'
                  : 'text-gray-400 hover:text-brand-sky hover:bg-white/5',
              )}
            >
              {item.name}
            </Link>
          ))}
        </nav>
      </aside>

      {/* Main Content */}
      <main className="flex-1 ml-64 min-h-screen bg-background text-brand-snow overflow-auto">
        <div className="container mx-auto p-8 max-w-screen-xl">
          <Outlet />
        </div>
      </main>
    </div>
  );
};

export default Layout;
