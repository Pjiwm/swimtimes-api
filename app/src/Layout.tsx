import React from 'react';
import Footer from './components/Footer';
import Navbar from './components/Navbar';

interface Props {
    children: React.ReactNode;
}

const Layout: React.FC<Props> = ({ children }) => {
    return (
        <div className="flex flex-col min-h-screen">
            <Navbar />
            <main className="flex-grow">
                {/* Header component can go here if needed */}
                {children}
            </main>
            <Footer />
        </div>
    );
};

export default Layout;