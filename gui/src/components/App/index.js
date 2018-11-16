import React, { Component } from "react";
import Background from "../Background";
import * as rpc from "../../utils/rpc";

import LOGO from "../../static/images/logo.png";

import "./index.scss";

class App extends Component {
  state = {
    state: "init",
    error: "",
    version: ""
  };

  componentDidMount() {
    rpc.addEventListener("stateChange", arg => {
      const { state, version, error } = arg;
      const s = { state };
      if (version) s.version = version;
      if (error) s.error = error;
      this.setState(s);
    });
  }

  get stateText() {
    const textMap = {
      ok: "Ok",
      error: "Failed",
      init: "Initialing...",
      download: "Downloading runtime...",
      unzip: "Unzip runtime...",
      install: "Install runtime..."
    };
    return textMap[this.state.state];
  }

  render() {
    rpc.log("render");
    return (
      <div className="App">
        <div className="main">
          <img className="logo" src={LOGO} alt="" />
          <p>Electron Platform</p>
          <p className="state-text">{this.stateText}</p>
        </div>
        <Background />
      </div>
    );
  }
}

export default App;
