import React, { useState } from 'react';
import './App.css';
import Login from './Login';
import RoomStatus from './RoomStatus';

function App() {
  const [username, setUsername] = useState<string | null>(null);

  return (
    <div className="App">
      {username ? (
        <RoomStatus username={username} />
      ) : (
        <Login onLogin={setUsername} />
      )}
    </div>
  );
}

export default App;