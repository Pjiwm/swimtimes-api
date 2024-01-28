// Navbar.tsx

import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBars, faTimes } from '@fortawesome/free-solid-svg-icons';

interface NavLinkProps {
  to: string;
  children: React.ReactNode;
  onClick?: () => void; // Make onClick optional
}

const Navbar: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const handleToggleMenu = () => {
    setIsMenuOpen(!isMenuOpen);
  };

  return (
    <nav className="bg-blue-700 p-4">
      <div className="flex items-center justify-between flex-wrap">
        {/* Logo */}
        <Link to="/" className="text-white text-4xl font-extrabold mb-4 md:mb-0">
          Swimtimes
        </Link>

        {/* Hamburger Menu for Mobile */}
        <div className="md:hidden">
          <button
            onClick={handleToggleMenu}
            className="text-white focus:outline-none"
          >
            {isMenuOpen ? (
              <FontAwesomeIcon icon={faTimes} />
            ) : (
              <FontAwesomeIcon icon={faBars} />
            )}
          </button>
        </div>

        {/* Navigation Links */}
        <div className={`md:flex flex-col md:flex-row items-start md:items-center space-y-2 md:space-y-0 md:space-x-4 w-full md:w-auto ${isMenuOpen ? 'flex' : 'hidden'}`}>
          <NavLink to="/" onClick={handleToggleMenu}>Home</NavLink>
          <NavLink to="/teams" onClick={handleToggleMenu}>Teams</NavLink>
          <NavLink to="/competitions" onClick={handleToggleMenu}>Competitions</NavLink>
          <NavLink to="/swimmers" onClick={handleToggleMenu}>Swimmers</NavLink>
          <NavLink to="/about" onClick={handleToggleMenu}>About</NavLink>
        </div>
      </div>
    </nav>
  );
};

const NavLink: React.FC<NavLinkProps> = ({ to, children, onClick }) => {
  return (
    <Link
      to={to}
      className="text-white px-4 py-2 border-b-2 border-transparent hover:border-white transition duration-300"
      onClick={onClick}
    >
      {children}
    </Link>
  );
};

export default Navbar;
