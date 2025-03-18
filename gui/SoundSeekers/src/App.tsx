import React, { Component } from 'react';
import {View} from 'react-native';
import windowStyles from './styles/WindowStyles';
import OpenSheetMusicDisplay from 'opensheetmusicdisplay';

class App extends Component {
  constructor(props: {}) {
    super(props);
    // Don't call this.setState() here!
    this.state = { file: "MuzioClementi_SonatinaOpus36No1_Part2.xml" };
  }

  handleClick(event: { target: { value: any; }; }) {
    const file = event.target.value;
    this.setState(state => state.file = file);
  }

  render() {
    return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <h1 className="App-title">OpenSheetMusicDisplay in React</h1>
        </header>
        <select onChange={this.handleClick.bind(this)}>
          <option value="MuzioClementi_SonatinaOpus36No1_Part2.xml">Muzio Clementi: Sonatina Opus 36 No1 Part2</option>
          <option value="Beethoven_AnDieFerneGeliebte.xml">Beethoven: An Die FerneGeliebte</option>
        </select>
        <OpenSheetMusicDisplay file={this.state.file} />
      </div>
    );
  }
}

export default App;
