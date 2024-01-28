import React from 'react';
import { useQuery } from '@apollo/client';
import { GET_TEAMS_BY_NAME } from '../queries';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner, faMapMarker } from '@fortawesome/free-solid-svg-icons';

const Teams: React.FC = () => {
  const { loading, error, data } = useQuery(GET_TEAMS_BY_NAME, {
    variables: { name: '' },
  });

  if (loading) return <div className="text-center mt-8"><FontAwesomeIcon icon={faSpinner} spin size="2x" className="text-white" /></div>;
  if (error) return <div className="text-center mt-8">Error: {error.message}</div>;

  const teams: Array<any> = data.getTeamsByName;

  return (
    <div className="max-w-screen-md mx-auto mt-8">
      <h1 className="text-4xl font-bold mb-4 text-white">Teams</h1>
      <div className="flex flex-col space-y-4">
        {teams.map((team: any, index: number) => (
          <div key={index} className="bg-white shadow-md p-4 rounded-lg">
            <div className="flex flex-col lg:flex-row items-start lg:items-center justify-between">
              <div>
                <p className="text-lg font-semibold text-blue-700">{team.name}</p>
                <p className="text-gray-600 mb-2">Address: {team.address}</p>
              </div>
              <div className="flex items-start space-x-2 lg:ml-auto">
                <FontAwesomeIcon icon={faMapMarker} className="text-blue-700 mt-1" />
                <div className="text-gray-600">
                  <div>{team.address}</div>
                  <div>Zip Code: {team.zipCode}</div>
                </div>
              </div>
            </div>
            <hr className="my-2" />
          </div>
        ))}
      </div>
    </div>
  );
};

export default Teams;
