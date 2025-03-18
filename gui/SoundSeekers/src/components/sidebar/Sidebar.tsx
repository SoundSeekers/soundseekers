import React from 'react';
import {View} from 'react-native';
import windowStyles from '../../styles/WindowStyles';
import SidebarIcon from './SidebarIcon';

const Sidebar = () => {
  return (
    <View style={[windowStyles.sidebar]}>
      <SidebarIcon icon="" title="Library" />
      <SidebarIcon icon="" title="Piano Rolls" />
      <SidebarIcon icon="" title="Note Sheets" />
    </View>
  );
};

export default Sidebar;
