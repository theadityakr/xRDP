// import React, { useContext } from "react";
import { PiMonitorLight } from "react-icons/pi";

import "../styles/form.css";

const Devices = () => {


    return (
        <div className="flex-column active-devices-super-container">
        <h2>Active Devices</h2>
       <div className="device-container">
           
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 1</p>
                </div>

                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 2</p>
                </div>
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 3</p>
                </div>
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 4</p>
                </div>
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 5</p>
                </div>
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 6</p>
                </div> 
                <div className="device">
                <PiMonitorLight  size={60}/>
                <p>Device 7</p>
                </div>          
            </div>
        </div>
    );
};

export default Devices;
