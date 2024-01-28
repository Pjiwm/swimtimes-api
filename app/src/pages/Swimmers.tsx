import React, { useState } from 'react';
import { useQuery } from '@apollo/client';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner, faSearch, faUser, faUserCircle, faUsers, faCalendar } from '@fortawesome/free-solid-svg-icons';
import { GET_SWIMMERS_BY_NAME } from '../queries';

const Swimmer: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [currentPage, setCurrentPage] = useState(1);

  const { loading, error, data, refetch } = useQuery(
    GET_SWIMMERS_BY_NAME,
    {
      variables: { name: searchTerm, index: (currentPage - 1) },
    }
  );

  const handleSearch = () => {
    setCurrentPage(1);
    refetch({ name: searchTerm }); // Update the query with the current search term
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  if (loading) return <div className="text-center mt-8"><FontAwesomeIcon icon={faSpinner} spin size="2x" className="text-white" /></div>;
  if (error) return <div className="text-center mt-8">Error: {error.message}</div>;

  // Check if data exists before accessing properties
  const swimmers = data ? data.getSwimmersByName : [];

  return (
    <div className="max-w-screen-xl mx-auto mt-8">
      {/* Search bar at the top */}
      <div className="p-4 bg-gray-100 mb-4 flex items-center">
        <input
          type="text"
          value={searchTerm}
          placeholder="Search by swimmer name"
          className="p-4 border border-gray-300 rounded-md focus:outline-none focus:ring focus:border-blue-300 flex-grow"
          onKeyPress={handleKeyPress}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <button
          className="p-4 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:border-blue-300"
          onClick={handleSearch}
        >
          <FontAwesomeIcon icon={faSearch} />
        </button>
      </div>

      {/* Swimmers List */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {swimmers.map((swimmer: any, index: number) => (
          <div key={index} className="bg-white shadow-md p-4 rounded-lg mb-4">
            <div className="flex flex-col items-start justify-between h-full">
              <div>
                <p className="text-lg font-semibold text-blue-700">
                  <FontAwesomeIcon icon={faUser} /> ID: {swimmer.id}
                </p>
                <p className="text-lg font-semibold text-blue-700">
                  <FontAwesomeIcon icon={faUserCircle} /> Name: {swimmer.name}
                </p>
                <p className="text-lg font-semibold text-blue-700">
                  <FontAwesomeIcon icon={faUsers} /> Team: {swimmer.team}
                </p>
                <p className="text-lg font-semibold text-blue-700">
                  <FontAwesomeIcon icon={faCalendar} /> Date of Birth: {new Date(swimmer.dateOfBirth).toLocaleDateString('en-US')}
                </p>
              </div>
              <hr className="my-2" />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Swimmer;
