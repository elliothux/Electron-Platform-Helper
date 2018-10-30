import React, { Component } from 'react';
import Background from '../Background';

import LOGO from '../../static/images/logo.png';

import './index.scss';

class App extends Component {
  render() {
    return (
      <div className="App">
        <div className="main">
          <img className="logo" src={LOGO} alt="" />
          <p>Electron Platform</p>
        </div>
        <Background/>
      </div>
    );
  }
}

export default App;
