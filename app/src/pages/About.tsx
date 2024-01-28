import React from 'react';
import { Link } from 'react-router-dom';

const About: React.FC = () => {
  return (
    <div className="flex flex-col items-center mt-8">
      <h1 className="text-4xl font-bold mb-4">About Swimtimes</h1>
      <div className="rounded-lg p-6 max-w-screen-md w-full text-left">
        <p className="text-xl text-white">
          Swimtimes is your go-to platform for exploring swim times and results for teams, competitions, and swimmers.
        </p>

        <div className="mt-8">
          <p className="text-xl text-white">
            Whether you're interested in tracking your favorite team's performance, staying updated on the latest
            competitions, or following the achievements of individual swimmers, Swimtimes provides a comprehensive
            overview of swim-related data.
          </p>
        </div>

        <div className="mt-8">
          <p className="text-xl text-white">
            If you're a manager, want to manage teams, swim times, and competition data you can{' '}
            <Link to="/login" className="text-blue-500 underline">
              login or register via this link
            </Link>
            .
          </p>
        </div>
      </div>
    </div>
  );
};

export default About;
