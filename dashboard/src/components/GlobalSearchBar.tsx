
import React from 'react';

const GlobalSearchBar: React.FC = () => {
  return (
    <div className="mb-6">
      <input
        type="text"
        placeholder="🔍 Global Search"
        className="w-full p-3 rounded-lg border border-gray-300 shadow-sm focus:outline-none focus:ring focus:border-blue-300"
      />
    </div>
  );
};

export default GlobalSearchBar;
