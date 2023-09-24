import React from 'react';
import ReactDOM from 'react-dom';

import App from './App';
import './assets/index.css';

// When creating the wallet you can optionally ask to create an access key
// Having the key enables to call non-payable methods without interrupting the user to sign
// const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS });

// Setup on page load
window.onload = async () => {
  ReactDOM.render(
    <App />,
    document.getElementById('root')
  );
};
