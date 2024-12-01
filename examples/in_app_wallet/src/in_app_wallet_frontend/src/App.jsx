import { useState } from 'react';
import { authManager } from './auth';
import { useEffect } from 'react';
import './App.css';

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [activeTab, setActiveTab] = useState('balance');
  const [principalID, setPrincipalID] = useState('');
  const [transferStatus, setTransferStatus] = useState('');
  const [isTransferring, setIsTransferring] = useState(false);
  const [balances, setBalances] = useState({
    ICP: '0.00',
    INWT: '0.00'
  });

  useEffect(() => {
    initAuth();
  }, []);

  const handleTransfer = async (event) => {
    event.preventDefault();
    setIsTransferring(true);
    setTransferStatus('');

    const recipientAddress = event.target.elements.address.value;
    const amount = parseFloat(event.target.elements.amount.value);
    const selectedToken = event.target.elements.token.value;

    try {
      let blockHeight;
      if (selectedToken === 'ICP') {
        blockHeight = await authManager.transferICP(recipientAddress, amount);
      } else if (selectedToken === 'INWT') {
        blockHeight = await authManager.transferINWT(recipientAddress, amount);
      }
      
      setTransferStatus(`Transfer successful! Block height: ${blockHeight}`);
      await updateBalances();
    } catch (error) {
      setTransferStatus(`Transfer failed: ${error.message}`);
    } finally {
      setIsTransferring(false);
    }
  };

  const initAuth = async () => {
    const isAuthenticated = await authManager.init();
    setIsLoggedIn(isAuthenticated);
    if (isAuthenticated) {
      await updateBalances();
      getIdentity();
    }
  };

  const updateBalances = async () => {
    const balances = await authManager.getBalances();
    setBalances({
      ICP: (Number(balances.ICP) / 100000000).toFixed(4),
      INWT: (Number(balances.INWT) / 100000000).toFixed(4)
    });
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
         <form className="transfer-form" onSubmit={handleTransfer}>
           <select className="token-select" name="token">
             <option value="ICP">ICP</option>
             <option value="INWT">INWT</option>
           </select>
           <input 
             type="text" 
             placeholder="Recipient Principal ID"
             className="address-input"
             name="address"
             required
           />
           <input 
             type="number" 
             placeholder="Amount"
             className="amount-input"
             name="amount"
             min="0"
             step="0.00000001"
             required
           />
           <button 
             type="submit" 
             className="send-button"
             disabled={isTransferring}
           >
             {isTransferring ? 'Sending...' : 'Send Tokens'}
           </button>
           {transferStatus && (
             <div className={`transfer-status ${transferStatus.includes('failed') ? 'error' : 'success'}`}>
               {transferStatus}
             </div>
           )}
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