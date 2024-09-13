import React, { useState,useEffect } from "react";
import { LuMonitorDot } from "react-icons/lu";
import { GoHome } from "react-icons/go";
import { TbLayoutSidebarLeftExpand } from "react-icons/tb";
import { IconContext } from "react-icons";

import '../styles/sidebar.css';

// Define the type for the props
interface SidebarProps {
    onTabChange: (tab: "Home" | "Devices") => void;
    activeTab: "Home" | "Devices";
}

const Sidebar: React.FC<SidebarProps> = ({ onTabChange, activeTab }) => {
    const [isExpanded, setIsExpanded] = useState(true);

    useEffect(() => {
        const handleResize = () => {
            if (window.innerWidth < 700) {
                setIsExpanded(true);
            } else {
                setIsExpanded(false);
            }
        };

        window.addEventListener('resize', handleResize);
        // Set initial state based on window width
        handleResize();

        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);

    const handleClick = (tab: "Home" | "Devices") => {
        onTabChange(tab);
    };

    const toggleSidebar = () => {
        setIsExpanded(!isExpanded);
    };

    return (
        <div className={`sidebar ${isExpanded ? 'expanded' : 'collapsed'}`}>
            <div className="sidebar-container">
                <div className="sidebar-tile" onClick={toggleSidebar}>
                    <IconContext.Provider value={{ className: "sidebarIcon" }}>
                        <TbLayoutSidebarLeftExpand size={25} />
                        <span>{isExpanded ? 'Collapse Sidebar' : 'Expand Sidebar'}</span>
                    </IconContext.Provider>
                </div>
                <div className={`sidebar-item ${activeTab === "Home" ? 'active' : ''}`} onClick={() => handleClick("Home")}>
                    <GoHome size={24} />
                    <span>Home</span>
                </div>
                <div className={`sidebar-item ${activeTab === "Devices" ? 'active' : ''}`} onClick={() => handleClick("Devices")}>
                    <LuMonitorDot size={24} />
                    <span>Devices</span>
                </div>
            </div>
        </div>
    );
};

export default Sidebar;
