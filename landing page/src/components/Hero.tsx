import React from 'react';
import { ArrowRight, Shield } from 'lucide-react';

export function Hero() {
  return (
    <section className="pt-32 pb-20 px-6 bg-brand-yellow/30">
      <div className="max-w-7xl mx-auto">
        <div className="text-center max-w-3xl mx-auto">
          <h1 className="text-5xl font-bold text-brand-blue mb-6">
            Secure Peer-to-Peer File Sharing Made Simple
          </h1>
          <p className="text-xl text-gray-600 mb-8">
            Transfer files directly between devices with end-to-end encryption.
            No servers, no limits, just secure sharing.
          </p>
          <div className="flex items-center justify-center gap-4">
            <button className="bg-brand-blue text-white px-6 py-3 rounded-lg hover:bg-brand-blue/90 transition-colors flex items-center gap-2">
              Start Sharing Now
              <ArrowRight className="w-5 h-5" />
            </button>
            <button className="border border-brand-blue/20 text-brand-blue px-6 py-3 rounded-lg hover:bg-brand-blue/5 transition-colors flex items-center gap-2">
              Learn More
            </button>
          </div>
          <div className="mt-8 flex items-center justify-center gap-2 text-gray-600">
            <Shield className="w-5 h-5 text-brand-blue" />
            <span>End-to-end encrypted and completely private</span>
          </div>
        </div>
      </div>
    </section>
  );
}