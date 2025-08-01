import { useAuth } from '@/providers/AuthProvider';
import { Navigate, Outlet } from 'react-router';

export default () => {
  const { user } = useAuth();

  if (!user) return <Navigate to={'/auth/login'} />;

  return <Outlet />;
};
