import { PageLayout } from '@/components/layout/PageLayout';
import { PageHeader } from '@/components/layout/PageHeader';
import { PageSection } from '@/components/layout/PageSection';

const About = () => {
  return (
    <PageLayout>
      <PageHeader title="About" />
      <PageSection>
        <div className="max-w-2xl space-y-3 text-sm leading-relaxed">
          <p className="text-muted-foreground">
            Heimdall is developed by the{' '}
            <strong className="text-white/90">Universidad Politécnica de Madrid (UPM)</strong> as
            part of the EUNOMIA project.
          </p>
          <p className="text-muted-foreground">
            You can find out more about us and the project at{' '}
            <a
              href="https://eunomia.dit.upm.es"
              target="_blank"
              rel="noopener noreferrer"
              className="text-brand-sky underline underline-offset-4 hover:opacity-80 transition-opacity"
            >
              eunomia.dit.upm.es
            </a>
            .
          </p>
        </div>
      </PageSection>
    </PageLayout>
  );
};

export default About;
