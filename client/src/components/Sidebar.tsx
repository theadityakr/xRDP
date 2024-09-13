import React, { useState } from "react";
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
                        <TbLayoutSidebarLeftExpand size={21} />
                        <span>{isExpanded ? 'Collapse Sidebar' : 'Expand Sidebar'}</span>
                    </IconContext.Provider>
                </div>
                <div className={`sidebar-item ${activeTab === "Home" ? 'active' : ''}`} onClick={() => handleClick("Home")}>
                    <GoHome size={20} />
                    <span>Home</span>
                </div>
                <div className={`sidebar-item ${activeTab === "Devices" ? 'active' : ''}`} onClick={() => handleClick("Devices")}>
                    <LuMonitorDot size={20} />
                    <span>Devices</span>
                </div>
            </div>
        </div>
    );
};

export default Sidebar;
