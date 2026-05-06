import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';

const Dashboard = () => {
  return (
    <PageLayout>
      <PageHeader title="Dashboard" />
      <PageSection>
        <p className="text-muted-foreground text-sm">Interact with the system here.</p>
      </PageSection>
    </PageLayout>
  );
};

export default Dashboard;
