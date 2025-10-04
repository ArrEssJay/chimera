function App() {
  return (
    <div className="app">
      <header>
        <h1>Chimera - React + TypeScript</h1>
        <p>Signal Processing Workbench</p>
      </header>
      
      <main>
        <section className="panel">
          <h2>Welcome</h2>
          <p>
            This is the new React + TypeScript frontend for Chimera.
            The infrastructure is set up and ready for component development.
          </p>
          <ul>
            <li>✅ React 18 + TypeScript 5.5 + Vite 5.4</li>
            <li>✅ Vitest + React Testing Library</li>
            <li>✅ Storybook 8.2</li>
            <li>✅ Recharts 2.12</li>
            <li>✅ Zustand 4.5</li>
          </ul>
        </section>
      </main>
    </div>
  );
}

export default App;
