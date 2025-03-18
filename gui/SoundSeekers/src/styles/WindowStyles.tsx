import {StyleSheet} from 'react-native';

const windowStyles = StyleSheet.create({
  backgroud: {
    flex: 1,
    flexDirection: 'row',
  },
  main: {
    flex: 0.95,
    backgroundColor: '#ff8da1',
  },
  sidebar: {
    flex: 0.1,
    gap: 10,
    minWidth: 30,
    maxWidth: 100,
    padding: 5,
    margin: 10,
    backgroundColor: 'rgba(255, 255, 255, 0.05)',
    borderRadius: 20,
    borderColor: 'rgba(255, 255, 255, 0.19)',
    borderWidth: 1,
    borderStyle: 'solid'
  },
});

export default windowStyles;
