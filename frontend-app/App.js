import 'regenerator-runtime/runtime';
import React, { useEffect } from 'react';

import { Configurator, Wallet, Logo } from './components';
import './assets/index.css';

const createAccessKeyFor = process.env.CONTRACT_NAME;

export default function App() {
  const wallet = new Wallet({
    createAccessKeyFor,
  });

  const handleNEARLoginOnStartup = async () => {
    const signedIn = await wallet.startUp();

    if (!signedIn) {
      wallet.signIn();
    }
  };

  useEffect(() => {
    handleNEARLoginOnStartup();
  }, []);

  Logo('logo-container');

  return (
    <main>
      <Configurator />
    </main>
  );
};
