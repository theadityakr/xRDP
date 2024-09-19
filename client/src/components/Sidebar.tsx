import React, { useState, useEffect } from "react";
import { LuMonitorDot } from "react-icons/lu";
import { TbSmartHome } from "react-icons/tb";
import { TbLayoutSidebarLeftExpand } from "react-icons/tb";
import { IconContext } from "react-icons";

import '../styles/sidebar.css';

interface SidebarProps {
    onTabChange: (tab: "Home" | "Devices") => void;
    activeTab: "Home" | "Devices";
}

const Sidebar: React.FC<SidebarProps> = ({ onTabChange, activeTab }) => {
    const [isExpanded, setIsExpanded] = useState(true);

    // useEffect(() => {
    //     const handleResize = () => {
    //         if (window.innerWidth < 700) {
    //             setIsExpanded(true);
    //         } else {
    //             setIsExpanded(false);
    //         }
    //     };

    //     window.addEventListener('resize', handleResize);
    //     handleResize();

    //     return () => {
    //         window.removeEventListener('resize', handleResize);
    //     };
    // }, []);

    const handleClick = (tab: "Home" | "Devices") => {
        onTabChange(tab);
    };

    const toggleSidebar = () => {
        setIsExpanded(!isExpanded);
    };

    return (
        <div className={`sidebar ${isExpanded ? 'expanded' : 'collapsed'}`}>
            <div className="sidebar-container">
                <div className="sidebar-item sidebar-tile" onClick={toggleSidebar}>
                    <IconContext.Provider value={{ className: "sidebarIcon" }}>
                        <TbLayoutSidebarLeftExpand size={28} />
                        <span>{isExpanded ? '' : 'Expand Sidebar'}</span>
                    </IconContext.Provider>
                </div>
                <div className={`sidebar-item ${activeTab === "Home" ? 'active' : ''}`} onClick={() => handleClick("Home")}>
                    <TbSmartHome size={27} />
                    <span>Home</span>
                </div>
                <div className={`sidebar-item ${activeTab === "Devices" ? 'active' : ''}`} onClick={() => handleClick("Devices")}>
                    <LuMonitorDot size={26} />
                    <span>Devices</span>
                </div>
            </div>
        </div>
    );
};

export default Sidebar;
