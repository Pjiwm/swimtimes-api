import React from 'react';

const NotFound: React.FC = () => {
  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <h1 className="text-9xl font-bold text-blue-600">404</h1>
      <p className="text-lg mt-4">Page not found</p>
    </div>
  );
};

export default NotFound;