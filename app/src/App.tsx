import './App.css'
import Login from './Login'
import Layout from './Layout'
import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Teams from './pages/Teams';
import Competitions from './pages/Competions';
import Swimmers from './pages/Swimmers';
import About from './pages/About';
import Home from './pages/Home';
function App() {


  // <Login />
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/teams" element={<Teams />} />
          <Route path="/competitions" element={<Competitions />} />
          <Route path="/swimmers" element={<Swimmers />} />
          <Route path="/about" element={<About />} />
          <Route path="/login" element={<Login />} />
        </Routes>
      </Layout>
    </Router>
  )
}

export default App
