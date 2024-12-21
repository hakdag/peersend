import React from 'react';
import { ArrowRight } from 'lucide-react';

export function HowItWorks() {
  return (
    <section id="how-it-works" className="py-20 px-6 bg-brand-yellow/20">
      <div className="max-w-7xl mx-auto">
        <h2 className="text-3xl font-bold text-center text-brand-blue mb-12">
          How PeerSend Works
        </h2>
        <div className="grid md:grid-cols-3 gap-8">
          {[
            {
              step: '01',
              title: 'Select Files',
              description: 'Choose the files you want to share from your device',
            },
            {
              step: '02',
              title: 'Generate Link',
              description: 'Get a secure sharing link for your recipient',
            },
            {
              step: '03',
              title: 'Transfer',
              description: 'Recipient opens the link and receives files directly',
            },
          ].map((item, index) => (
            <div key={index} className="relative">
              <div className="bg-white rounded-lg p-6 border border-brand-blue/10">
                <span className="text-5xl font-bold text-brand-yellow">
                  {item.step}
                </span>
                <h3 className="text-xl font-semibold text-brand-blue mt-4 mb-2">
                  {item.title}
                </h3>
                <p className="text-gray-600">{item.description}</p>
              </div>
              {index < 2 && (
                <ArrowRight className="hidden md:block absolute top-1/2 -right-4 w-8 h-8 text-brand-blue/30 transform -translate-y-1/2" />
              )}
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}