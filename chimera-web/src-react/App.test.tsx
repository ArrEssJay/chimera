import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import App from './App';

describe('App', () => {
  it('renders without crashing', () => {
    render(<App />);
    expect(screen.getByText('Chimera - React + TypeScript')).toBeInTheDocument();
  });

  it('has skip navigation link', () => {
    render(<App />);
    const skipLink = screen.getByText('Skip to main content');
    expect(skipLink).toBeInTheDocument();
    expect(skipLink).toHaveAttribute('href', '#main-content');
  });

  it('has proper semantic structure', () => {
    const { container } = render(<App />);
    
    // Check for banner
    expect(screen.getByRole('banner')).toBeInTheDocument();
    
    // Check for main content
    expect(screen.getByRole('main')).toBeInTheDocument();
    expect(screen.getByRole('main')).toHaveAttribute('id', 'main-content');
    
    // Check for footer
    expect(screen.getByRole('contentinfo')).toBeInTheDocument();
  });

  it('has proper ARIA labels', () => {
    render(<App />);
    
    // Check for labeled section
    expect(screen.getByLabelText('Technology stack')).toBeInTheDocument();
  });

  it('has heading hierarchy', () => {
    render(<App />);
    
    // H1 heading
    expect(screen.getByRole('heading', { level: 1, name: 'Chimera - React + TypeScript' }))
      .toBeInTheDocument();
    
    // H2 heading
    expect(screen.getByRole('heading', { level: 2, name: 'Welcome' }))
      .toBeInTheDocument();
  });

  it('has accessible links', () => {
    render(<App />);
    
    const githubLink = screen.getByRole('link', { name: 'GitHub' });
    expect(githubLink).toBeInTheDocument();
    expect(githubLink).toHaveAttribute('target', '_blank');
    expect(githubLink).toHaveAttribute('rel', 'noopener noreferrer');
  });
});
