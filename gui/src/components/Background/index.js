import React from 'react';

import BG1 from '../../static/images/bg1.png';
import BG2 from '../../static/images/bg2.png';

import "./index.scss";


function Background() {
  return (
    <div id="background">
      <img src={BG1} alt="" />
      <img src={BG2} alt="" />
    </div>
  );
}


export default Background;
