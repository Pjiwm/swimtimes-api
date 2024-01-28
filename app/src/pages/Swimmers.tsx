import React, { useState, } from 'react';
import { useQuery } from '@apollo/client';
import { GET_TEAMS_BY_NAME } from '../queries';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner, faMapMarker, faSearch, faTimes, faChevronLeft, faChevronRight } from '@fortawesome/free-solid-svg-icons';

const Teams: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [searchTermOnSearch, setSearchTermOnSearch] = useState('');
  const [currentPage, setCurrentPage] = useState(1);
  const { loading, error, data, refetch } = useQuery(GET_TEAMS_BY_NAME, {
    variables: { name: searchTermOnSearch, index: (currentPage - 1) },
  });

  const [pageSize, _] = useState(data.getTeamsByName.length);


  const handleSearch = () => {
    setCurrentPage(1); // Reset to the first page when searching
    setSearchTermOnSearch(searchTerm);
  };

  const resetFilter = () => {
    setSearchTerm('');
    setSearchTermOnSearch('');
    setCurrentPage(1); // Reset to the first page when resetting filter
    refetch(); // Explicitly refetch data after resetting the filter
  };

  const handlePrevPage = () => {
    if (currentPage > 1) {
      setCurrentPage(currentPage - 1);
    }
  };

  const handleNextPage = async () => {
    if (data.getTeamsByName.length != pageSize) {
      return;
    }

    const response = await refetch({ name: searchTermOnSearch, index: currentPage /*Not -1 since we use an idx */ });
    if (response.data.getTeamsByName.length == 0) {
      await refetch({ name: searchTermOnSearch, index: currentPage - 1 });
      return;
    };

    setCurrentPage(currentPage + 1);
  };

  if (loading) return <div className="text-center mt-8"><FontAwesomeIcon icon={faSpinner} spin size="2x" className="text-white" /></div>;
  if (error) return <div className="text-center mt-8">Error: {error.message}</div>;

  let teams: Array<any> = data.getTeamsByName;

  return (
    <div className="max-w-screen-xl mx-auto mt-8 flex flex-col md:flex-row">
      {/* Filter Card on the left */}
      <div className="w-full md:w-1/4 h-1/4 p-4 bg-gray-100 md:flex-shrink-0 mb-4">
        <h2 className="text-2xl font-bold mb-2 flex items-center justify-between">
          Filter
          <button
            className="text-blue-500 hover:text-blue-700 focus:outline-none"
            onClick={resetFilter}
          >
            <FontAwesomeIcon icon={faTimes} className="ml-1" />
          </button>
        </h2>
        {/* Search bar (Rendered inline on larger screens) */}
        <div className="flex items-center mb-4 md:flex">
          <div className="flex-grow">
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search by team name"
              className="p-2 border border-gray-300 rounded-l-md focus:outline-none focus:ring focus:border-blue-300 w-full"
            />
          </div>
          {/* Search button */}
          <button
            className="p-2 bg-blue-500 text-white rounded-r-md hover:bg-blue-600 focus:outline-none focus:ring focus:border-blue-300"
            onClick={handleSearch}
          >
            <FontAwesomeIcon icon={faSearch} />
          </button>
        </div>
        {/* Page selection controls */}
        <div className="flex items-center justify-between mb-4">
          <button
            className="p-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:border-blue-300"
            onClick={handlePrevPage}
            disabled={currentPage === 1}
          >
            <FontAwesomeIcon icon={faChevronLeft} />
          </button>
          <p className="text-gray-600">{`Page ${currentPage}`}</p>
          <button
            className="p-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring focus:border-blue-300"
            onClick={handleNextPage}
          >
            <FontAwesomeIcon icon={faChevronRight} />
          </button>
        </div>
        {/* Display number of results */}
        <p className="text-gray-600">Results: {teams.length}</p>
      </div>

      {/* Teams List on the right */}
      <div className="w-full md:w-3/4 p-4 flex-grow bg-blue-600 rounded-md mt-4 md:ml-4">
        <h1 className="text-4xl font-bold mb-4 text-white">Teams</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {teams.map((team: any, index: number) => (
            <div key={index} className="bg-white shadow-md p-4 rounded-lg mb-4">
              <div className="flex flex-col items-start justify-between h-full">
                <div>
                  <p className="text-lg font-semibold text-blue-700">{team.name}</p>
                  <div className="flex items-start space-x-2 mt-2">
                    <FontAwesomeIcon icon={faMapMarker} className="text-blue-700 mt-1" />
                    <div className="text-gray-600">
                      <div>{team.address}</div>
                      <div>Zip Code: {team.zipCode}</div>
                    </div>
                  </div>
                </div>
                <hr className="my-2" />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default Teams;
