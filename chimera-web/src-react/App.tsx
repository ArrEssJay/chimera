function App() {
  return (
    <div className="app">
      {/* Skip navigation for accessibility */}
      <a href="#main-content" className="skip-to-main">
        Skip to main content
      </a>
      
      <header role="banner">
        <h1>Chimera - React + TypeScript</h1>
        <p>Signal Processing Workbench</p>
      </header>
      
      <main id="main-content" role="main">
        <section className="panel" aria-labelledby="welcome-heading">
          <h2 id="welcome-heading">Welcome</h2>
          <p>
            This is the new React + TypeScript frontend for Chimera.
            The infrastructure is set up and ready for component development.
          </p>
          <ul aria-label="Technology stack">
            <li>✅ React 18 + TypeScript 5.5 + Vite 5.4</li>
            <li>✅ Vitest + React Testing Library</li>
            <li>✅ Storybook 8.2</li>
            <li>✅ Recharts 2.12</li>
            <li>✅ Zustand 4.5</li>
          </ul>
        </section>
      </main>
      
      <footer role="contentinfo" className="app-footer">
        <div className="footer-content">
          <span>Chimera DSP System</span>
          <span className="footer-separator">•</span>
          <a href="https://github.com/ArrEssJay/chimera" target="_blank" rel="noopener noreferrer">
            GitHub
          </a>
        </div>
      </footer>
    </div>
  );
}

export default App;
