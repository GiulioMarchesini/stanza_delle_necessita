import React from 'react';

function Leaderboard() {
  const users = [
    { username: 'utente1', time: 120 },
    { username: 'utente2', time: 90 },
    { username: 'utente3', time: 60 },
  ];

  return (
    <table className="Leaderboard">
      <thead>
        <tr>
          <th>Nome utente</th>
          <th>Tempo (minuti)</th>
        </tr>
      </thead>
      <tbody>
        {users.map((user, index) => (
          <tr key={index}>
            <td>{user.username}</td>
            <td>{user.time}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default Leaderboard;