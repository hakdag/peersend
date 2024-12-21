import React from 'react';
import { Shield, Smartphone, Zap, Server, Lock, Users } from 'lucide-react';

const features = [
  {
    icon: Shield,
    title: 'End-to-End Encryption',
    description: 'Your files are encrypted before leaving your device, ensuring complete privacy.',
  },
  {
    icon: Smartphone,
    title: 'Cross-Platform',
    description: 'Works seamlessly across desktop and mobile devices.',
  },
  {
    icon: Zap,
    title: 'Lightning Fast',
    description: 'Direct peer-to-peer transfers for maximum speed.',
  },
  {
    icon: Server,
    title: 'No Servers',
    description: 'Files transfer directly between devices, never touching a server.',
  },
  {
    icon: Lock,
    title: 'Privacy First',
    description: 'No registration or personal information required.',
  },
  {
    icon: Users,
    title: 'Open Source',
    description: 'Transparent, community-driven development you can trust.',
  },
];

export function Features() {
  return (
    <section id="features" className="py-20 bg-brand-yellow/25 px-6">
      <div className="max-w-7xl mx-auto">
        <h2 className="text-3xl font-bold text-center text-brand-blue mb-12">
          Why Choose PeerSend?
        </h2>
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
          {features.map((feature, index) => (
            <div
              key={index}
              className="bg-white p-6 rounded-xl shadow-sm hover:shadow-md transition-shadow border border-brand-blue/5"
            >
              <feature.icon className="w-12 h-12 text-brand-blue mb-4" />
              <h3 className="text-xl font-semibold text-brand-blue mb-2">
                {feature.title}
              </h3>
              <p className="text-gray-600">{feature.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}