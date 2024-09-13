import React, { useContext } from "react";
import { LuMonitorCheck } from "react-icons/lu";

import "../styles/form.css";

const Devices = () => {


    return (
       <div className="flex-column dev-container">
            <h3>Active Devices</h3>
                <div className="flex-column devices-container ">
                <LuMonitorCheck size={60}/>
                </div>
            </div>
    );
};

export default Devices;
