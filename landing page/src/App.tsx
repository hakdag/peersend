import React from 'react';
import { Header } from './components/Header';
import { Hero } from './components/Hero';
import { Features } from './components/Features';
import { HowItWorks } from './components/HowItWorks';
import { Security } from './components/Security';
import { Footer } from './components/Footer';

function App() {
  return (
    <div className="min-h-screen bg-brand-yellow/20">
      <Header />
      <main>
        <Hero />
        <Features />
        <HowItWorks />
        <Security />
      </main>
      <Footer />
    </div>
  );
}

export default App;