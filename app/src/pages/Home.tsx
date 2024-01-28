import React from 'react';
import { Link } from 'react-router-dom';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faUsers, faTrophy, faSwimmer } from '@fortawesome/free-solid-svg-icons';

const Home: React.FC = () => {
  return (
    <div className="flex flex-col items-center mt-8">
      <div className="text-center">
        <h1 className="text-4xl font-bold mb-4">Welcome to Swimtimes!</h1>
        <p className="text-xl text-white">Explore swim times and results for teams, competitions, and swimmers.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-8 w-full max-w-screen-lg mx-auto px-3 py-5">
        <Link to="/teams" className="home-card rounded-lg overflow-hidden bg-white shadow-lg text-blue-700 flex flex-col items-center">
          <div className="card-content p-4 flex items-center justify-center">
            <FontAwesomeIcon icon={faUsers} size="3x" className="card-icon" />
          </div>
          <p className="card-text text-xl mt-2 text-black">Teams</p>
        </Link>

        <Link to="/competitions" className="home-card rounded-lg overflow-hidden bg-white shadow-lg text-blue-700 flex flex-col items-center ">
          <div className="card-content p-4 flex items-center justify-center">
            <FontAwesomeIcon icon={faTrophy} size="3x" className="card-icon" />
          </div>
          <p className="card-text text-xl mt-2 text-black">Competitions</p>
        </Link>

        <Link to="/swimmers" className="home-card rounded-lg overflow-hidden bg-white shadow-lg text-blue-700 flex flex-col items-center">
          <div className="card-content p-4 flex items-center justify-center">
            <FontAwesomeIcon icon={faSwimmer} size="3x" className="card-icon" />
          </div>
          <p className="card-text text-xl mt-2 text-black">Swimmers</p>
        </Link>
      </div>
    </div>
  );
};

export default Home;
