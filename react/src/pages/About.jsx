const About = () => {
  return (
    <div className="p-6 max-w-xl">
      <h1 className="text-2xl font-bold mb-4">About</h1>
      <p className="text-muted-foreground">
        Heimdall is developed by the <strong>Universidad Politécnica de Madrid (UPM)</strong> as
        part of the EUNOMIA project.
      </p>
      <p className="text-muted-foreground mt-3">
        You can find out more about us and the project at{' '}
        <a
          href="https://eunomia.dit.upm.es"
          target="_blank"
          rel="noopener noreferrer"
          className="text-primary underline underline-offset-4 hover:opacity-80 transition-opacity"
        >
          eunomia.dit.upm.es
        </a>
        .
      </p>
    </div>
  );
};

export default About;
