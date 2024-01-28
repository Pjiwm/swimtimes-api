const config = {
  apiUrl: process.env.NODE_ENV === 'development' ? 'http://localhost:8000' : 'https://swimtimes-api.shuttleapp.rs',
  authClientUrl: "https://hivfrefiwulvlhsjhzop.supabase.co",
  anonKey: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImhpdmZyZWZpd3Vsdmxoc2poem9wIiwicm9sZSI6ImFub24iLCJpYXQiOjE3MDYyMjcyNzEsImV4cCI6MjAyMTgwMzI3MX0.6edQB6SxlAcb-C-4t20jHCrKUq75h1Fz0cD1i6GW1tI"
};

export default config;