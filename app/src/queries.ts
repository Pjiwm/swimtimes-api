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

export const GET_SWIMMERS_BY_NAME = gql`
  query getSwimmersByName($name: String!, $index: Int!) {
    getSwimmersByName(name: $name, index: $index) {
      name
      team
      dateOfBirth
      id
    }
  }
`;
