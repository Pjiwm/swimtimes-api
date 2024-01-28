import { gql } from '@apollo/client';

export const GET_TEAMS_BY_NAME = gql`
  query GetTeamsByName($name: String!) {
    getTeamsByName(name: $name) {
		name,
        address,
        zipCode
    }
  }
`;
