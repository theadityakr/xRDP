import React, { useContext } from "react";
import { LuMonitorCheck } from "react-icons/lu";

import "../styles/form.css";

const Devices = () => {


    return (
        <div className="flex-column active-devices-super-contanier">
        <h2>Active Devices</h2>
       <div className="device-container">
           
                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 1</p>
                </div>

                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 2</p>
                </div>
                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 3</p>
                </div>
                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 4</p>
                </div>
                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 5</p>
                </div>
                <div className="device">
                <LuMonitorCheck size={60}/>
                <p>Device 6</p>
                </div>           
            </div>
        </div>
    );
};

export default Devices;
