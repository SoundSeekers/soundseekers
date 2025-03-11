import React from 'react';
import styles from './styles/WindowStyles';
import {View} from 'react-native';
import windowStyles from './styles/WindowStyles';

function App(): React.JSX.Element {
  return <View style={[windowStyles.window]}></View>;
}

export default App;
