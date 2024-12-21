import React from 'react';
import { Shield, Lock, Eye } from 'lucide-react';

export function Security() {
  return (
    <section id="security" className="py-20 bg-gradient-to-b from-brand-yellow/15 to-white px-6">
      <div className="max-w-7xl mx-auto">
        <div className="text-center max-w-3xl mx-auto mb-16">
          <h2 className="text-3xl font-bold text-brand-blue mb-4">
            Security You Can Trust
          </h2>
          <p className="text-gray-600">
            Your privacy and security are our top priorities. PeerSend uses cutting-edge
            WebRTC technology and encryption to ensure your files remain private.
          </p>
        </div>
        <div className="grid md:grid-cols-3 gap-8">
          {[
            {
              icon: Shield,
              title: 'End-to-End Encryption',
              description: 'All transfers are encrypted using state-of-the-art protocols',
            },
            {
              icon: Lock,
              title: 'No Data Storage',
              description: 'Files are never stored on any servers',
            },
            {
              icon: Eye,
              title: 'Complete Privacy',
              description: 'No tracking, no logging, just secure file transfers',
            },
          ].map((item, index) => (
            <div key={index} className="text-center">
              <div className="bg-white rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-4 shadow-sm border border-brand-blue/10">
                <item.icon className="w-8 h-8 text-brand-blue" />
              </div>
              <h3 className="text-xl font-semibold text-brand-blue mb-2">
                {item.title}
              </h3>
              <p className="text-gray-600">{item.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}