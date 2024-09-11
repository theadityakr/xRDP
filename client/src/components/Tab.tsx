import React, { useState } from 'react';
import { Tabs } from "@sinm/react-chrome-tabs";
import '@sinm/react-chrome-tabs/css/chrome-tabs.css';
import '@sinm/react-chrome-tabs/css/chrome-tabs-dark-theme.css';
import { IoMdAddCircleOutline,IoMdCloseCircle } from "react-icons/io";

import '../styles/tab.css'
import Button from './Button/Button';

export interface TabProperties {
  id: string;
  title: string;
  active?: boolean;
  // favicon class
  faviconClass?: string;
}

const Tab: React.FC <any> = () => {

  let id = 1;
  const [tabs, setTabs] = useState<TabProperties[]>([
    { id: "abc", title: "Tab", active: true },
  ]);

  const addTab = () => {
    id++;
    setTabs([
      ...tabs,
      {
        id: `tab-id-${id}`,
        title: `New Tab ${id}`
      },
    ]);
  };

  const active = (id: string) => {
    setTabs(tabs.map((tab) => ({ ...tab, active: id === tab.id })));
  };

  const close = (id: string) => {
    setTabs(tabs.filter((tab) => tab.id !== id));
  };

  const reorder = (tabId: string, fromIndex: number, toIndex: number) => {
    const beforeTab = tabs.find(tab => tab.id === tabId);
    if (!beforeTab) {
        return;
    }
    let newTabs = tabs.filter(tab => tab.id !== tabId);
    newTabs.splice(toIndex, 0, beforeTab);
    setTabs(newTabs);
  };

  const closeAll = () => setTabs([]);


  return (
      <div className='tab-bar flex-row'>
      
      <div className='tabs-container'>
      <Tabs
        darkMode={false}
        onTabClose={close}
        onTabReorder={reorder}
        onTabActive={active}
        tabs={tabs}
         
      ></Tabs>
      </div>
      <div className='flex-row tab-bar-buttons'>
        <button className='icons' onClick={addTab}><IoMdAddCircleOutline size={32}/></button>
        <button className='icons' onClick={closeAll}><IoMdCloseCircle size={32}/></button>
      </div>
    </div>
  );
};

export default Tab;
