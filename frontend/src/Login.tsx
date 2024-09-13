import React, { useState } from 'react';

interface LoginProps {
  onLogin: (username: string) => void;
}

function Login({ onLogin }: LoginProps) {
  const [username, setUsername] = useState('');

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    if (username) {
      onLogin(username);
    }
  };

  return (
    <div className="Login">
      <h2>Inserisci il tuo nome utente</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Nome utente"
        />
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}

export default Login;