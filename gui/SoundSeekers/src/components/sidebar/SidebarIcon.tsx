import React from 'react';
import { Text, View } from 'react-native';
import sideBarStyles from '../../styles/SideBarStyles';

type SidebarIconProps = {
    icon: string;
    title: string;
}

const SidebarIcon = (props: SidebarIconProps) => {
    return (
        <View style={[sideBarStyles.icon]}>
            <Text style={{textAlign:'center'}}>{props.title}</Text>
        </View>
    );
}

export default SidebarIcon;