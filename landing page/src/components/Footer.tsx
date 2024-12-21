import React from 'react';
import { Github } from 'lucide-react';
import { Logo } from './Logo';

export function Footer() {
  return (
    <footer className="bg-gray-900 text-gray-300 py-12 px-6">
      <div className="max-w-7xl mx-auto">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <Logo className="mb-6 md:mb-0" textClassName="text-xl text-white" />
          <div className="flex items-center gap-6">
            <a href="#" className="hover:text-white transition-colors">
              <Github className="w-6 h-6" />
            </a>
            <a href="#features" className="hover:text-white transition-colors">Features</a>
            <a href="#security" className="hover:text-white transition-colors">Security</a>
            <a href="#" className="hover:text-white transition-colors">Documentation</a>
          </div>
        </div>
        <div className="mt-8 pt-8 border-t border-gray-800 text-center text-sm">
          <p>© {new Date().getFullYear()} PeerSend. Open source software.</p>
        </div>
      </div>
    </footer>
  );
}