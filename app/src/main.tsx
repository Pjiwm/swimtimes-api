import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import { ApolloClient, InMemoryCache, ApolloProvider } from '@apollo/client';
import config from '../app.config.ts';
import './index.css'
import './styles/tailwind.css'

const client = new ApolloClient({

  uri: `${config.apiUrl}/api/graphql`,

  cache: new InMemoryCache(),

});

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <App />
    </ApolloProvider>
  </React.StrictMode>,
)
