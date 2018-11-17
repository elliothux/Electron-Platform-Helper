import React, { Component } from "react";
import Background from "../Background";

import LOGO from "../../static/images/logo.png";

import "./index.scss";

const { rpc } = window;

class App extends Component {
  state = {
    state: "",
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
    rpc.log("mounted");
    rpc.call("install");
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
    window.rpc.log("render");
    return (
      <div className="App">
        <div className="main">
          <img className="logo" src={LOGO} alt="" />
          <p>Electron Platform</p>
          <p className="state-text">{this.stateText}</p>
          <p
            className={`error-text ${
              this.state.state === "error" ? "show" : ""
            }`}
          >
            {this.state.error}
          </p>
        </div>
        <Background />
      </div>
    );
  }
}

export default App;
