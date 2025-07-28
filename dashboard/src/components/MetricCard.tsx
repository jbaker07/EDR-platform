// src/components/MetricCard.tsx
import React, { FC, ReactNode } from 'react';
import { motion } from 'framer-motion';

interface MetricCardProps {
  title: string;
  value: string;
  icon: ReactNode;
  color?: 'blue' | 'green' | 'red' | 'yellow' | 'gray';
}

const colorMap: Record<Required<MetricCardProps>['color'], string> = {
  blue: 'from-blue-800 to-blue-900',
  green: 'from-green-800 to-green-900',
  red: 'from-red-800 to-red-900',
  yellow: 'from-yellow-700 to-yellow-800',
  gray: 'from-gray-800 to-gray-900',
};

const MetricCard: FC<MetricCardProps> = ({ title, value, icon, color = 'gray' }) => {
  return (
    <motion.div
      className={`bg-gradient-to-br ${colorMap[color]} rounded-2xl p-6 shadow-xl text-white flex items-center justify-between`}
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      <div>
        <div className="text-sm opacity-70">{title}</div>
        <div className="text-2xl font-bold mt-1">{value}</div>
      </div>
      <div className="text-3xl opacity-50">{icon}</div>
    </motion.div>
  );
};

export default MetricCard;

