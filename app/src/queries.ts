import { gql } from '@apollo/client';

export const GET_TEAMS_BY_NAME = gql`
  query GetTeamsByName($name: String!, $index: Int!) {
    getTeamsByName(name: $name, index: $index) {
		name,
        address,
        zipCode
    }
  }
`;
