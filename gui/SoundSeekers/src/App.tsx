import React from 'react';
import {View} from 'react-native';

import windowStyles from './styles/WindowStyles';
import Sidebar from './components/sidebar/Sidebar';

function App(): React.JSX.Element {
  return (
    <View style={[windowStyles.backgroud]}>
      <Sidebar />
    </View>
  );
}

export default App;
