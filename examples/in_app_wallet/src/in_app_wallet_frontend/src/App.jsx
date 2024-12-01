import { useState } from 'react';
import { authManager } from './auth';
import { useEffect } from 'react';
import './App.css';

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [activeTab, setActiveTab] = useState('balance');
  const [principalID, setPrincipalID] = useState('');
  const [balances, setBalances] = useState({
    ICP: '0.00',
    INWT: '0.00'
  });

  useEffect(() => {
    initAuth();
  }, []);

  const initAuth = async () => {
    const isAuthenticated = await authManager.init();
    setIsLoggedIn(isAuthenticated);
    if (isAuthenticated) {
      await updateBalance();
      getIdentity();
    }
  };

  const updateBalance = async () => {
    const icpBalance = await authManager.getBalance();
    setBalances(prev => ({
      ...prev,
      ICP: (Number(icpBalance.toString()) / 100000000).toFixed(4)
    }));
  };

  const getIdentity = () => {
    const identity = authManager.getIdentity();
    const publicKey = identity.getPrincipal().toText(); 
    setPrincipalID(publicKey);
  };

  const handleLogin = async () => {
    const success = await authManager.login();
    setIsLoggedIn(success);
    if (success) {
      await updateBalance();
      getIdentity();
    }
  };

  const handleLogout = async () => {
    await authManager.logout();
    setIsLoggedIn(false);
    setPrincipalID('');
    setBalances({
      ICP: '0.00',
      INWT: '0.00'
    });
  };

  if (!isLoggedIn) {
    return (
      <div className="login-container">
        <h1>In-App Wallet</h1>
        <button className="login-button" onClick={handleLogin}>
          Login with Internet Identity
        </button>
      </div>
    );
  }

  return (
    <main className="wallet-container">
      <div className="wallet-header">
        <div className="header-top">
          <h1>In-App Wallet</h1>
          <button className="logout-button" onClick={() => handleLogout()}>
            Logout
          </button>
        </div>
        <div className="wallet-nav">
          <button 
            className={activeTab === 'balance' ? 'active' : ''} 
            onClick={() => setActiveTab('balance')}
          >
            Balance
          </button>
          <button 
            className={activeTab === 'send' ? 'active' : ''} 
            onClick={() => setActiveTab('send')}
          >
            Send
          </button>
          <button 
            className={activeTab === 'receive' ? 'active' : ''} 
            onClick={() => setActiveTab('receive')}
          >
            Receive
          </button>
        </div>
      </div>

      {activeTab === 'balance' && (
        <div className="balance-section">
          <div className="token-balance">
            <h3>ICP Balance</h3>
            <p className="amount">{balances.ICP} ICP</p>
          </div>
          <div className="token-balance">
            <h3>INWT Balance</h3>
            <p className="amount">{balances.INWT} INWT</p>
          </div>
        </div>
      )}

      {activeTab === 'send' && (
        <div className="send-section">
          <form className="transfer-form">
            <select className="token-select">
              <option value="ICP">ICP</option>
              <option value="INWT">INWT</option>
            </select>
            <input 
              type="text" 
              placeholder="Recipient Address"
              className="address-input"
            />
            <input 
              type="number" 
              placeholder="Amount"
              className="amount-input"
              min="0"
              step="0.01"
            />
            <button type="submit" className="send-button">
              Send Tokens
            </button>
          </form>
        </div>
      )}

      {activeTab === 'receive' && (
        <div className="receive-section">
          <h3>Your Wallet Address</h3>
          <div className="address-display">
            <code>{principalID }</code>
            <button className="copy-button">Copy</button>
          </div>
          <div className="qr-placeholder">
            [QR Code Placeholder]
          </div>
        </div>
      )}
    </main>
  );
}

export default App;