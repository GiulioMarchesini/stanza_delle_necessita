// come props un array di user
interface User {
  username: string;
  total_time: number;
}

interface LeaderboardProps {
  users: User[];
}

function Leaderboard({ users }: LeaderboardProps) {
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
            <td>{user.total_time}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

export default Leaderboard;
