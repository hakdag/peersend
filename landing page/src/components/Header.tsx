import React from 'react';
import { Logo } from './Logo';

export function Header() {
  return (
    <header className="w-full py-4 px-6 bg-white/90 backdrop-blur-sm fixed top-0 z-50 border-b border-gray-100">
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        <Logo textClassName="text-xl text-brand-blue" />
        <nav className="hidden md:flex items-center gap-8">
          <a href="#features" className="text-gray-600 hover:text-brand-blue transition-colors">Features</a>
          <a href="#how-it-works" className="text-gray-600 hover:text-brand-blue transition-colors">How it Works</a>
          <a href="#security" className="text-gray-600 hover:text-brand-blue transition-colors">Security</a>
        </nav>
        <a
          href="#"
          className="bg-brand-blue text-white px-4 py-2 rounded-lg hover:bg-brand-blue/90 transition-colors"
        >
          Start Sharing
        </a>
      </div>
    </header>
  );
}