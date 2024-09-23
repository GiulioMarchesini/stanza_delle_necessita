import React, { useState, useEffect } from "react";
import Leaderboard from "./Leaderboard";

const url = "http://192.168.0.130:9696";
// TODO ottieni l'url
// TODO metti tutti i tipi in un file a parte

interface RoomStatusProps {
  username: string;
}

interface User {
  username: string;
  total_time: number;
}

interface RoomState {
  is_free: boolean;
  current_user: string | null;
  start_time: number | null; // timestamp in secondi
  end_time: number | null; // timestamp in secondi
}

function RoomStatus({ username }: RoomStatusProps) {
  const [isRoomOccupied, setIsRoomOccupied] = useState(false);
  const [occupant, setOccupant] = useState<string | null>(null);
  const [leaderboard, setLeaderboard] = useState<User[]>([]);

  // fetch di stato stanza e chi la occupa. http request "/room_status"
  async function fetchRoomStatus() {
    const path: string = url + "/room_status";

    try {
      const response = await fetch(path);

      if (response.ok) {
        const result: RoomState = await response.json();
        console.log(result); // Stampa la risposta del server
        setIsRoomOccupied(!result.is_free);
        setOccupant(result.current_user);
      } else {
        console.error("Errore nella richiesta:", response.statusText);
      }
    } catch (error) {
      console.error("Errore:", error);
    }
  }

  // fetch classifica. get request "/leaderboard
  async function fetchLeaderboard() {
    const path: string = url + "/leaderboard";

    try {
      const response = await fetch(path);

      if (response.ok) {
        const result: User[] = await response.json();
        setLeaderboard(result);
        console.log(result); // Stampa la risposta del server
      } else {
        console.error("Errore nella richiesta:", response.statusText);
      }
    } catch (error) {
      console.error("Errore:", error);
    }
  }

  useEffect(() => {
    fetchRoomStatus();
    fetchLeaderboard();

    const interval = setInterval(() => {
      fetchRoomStatus();
      fetchLeaderboard();
    }, 15000);

    return () => clearInterval(interval);
  }, []);

  async function occupyRoom() {
    const path: string = url + "/occupy_room";
    console.log("path", path);
    const time = Math.floor(new Date().getTime() / 1000);
    const data: RoomState = {
      is_free: false,
      current_user: username,
      start_time: time,
      end_time: null,
    };
    console.log("data", data);

    try {
      const response = await fetch(path, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        const result = await response.json();
        console.log(result); // Stampa la risposta del server
        setIsRoomOccupied(true);
        setOccupant(username);
        alert(result); // Mostra un messaggio all'utente
      } else {
        console.error("Errore nella richiesta:", response.statusText);
        alert("Errore nella richiesta: " + response.statusText);
      }
    } catch (error) {
      console.error("Errore:", error);
      alert("Errore: " + (error as Error).message);
    }
  }

  async function freeRoom() {
    const path: string = url + "/free_room";
    const time = Math.floor(new Date().getTime() / 1000);
    const data: RoomState = {
      is_free: true,
      current_user: username,
      start_time: null,
      end_time: time,
    };

    try {
      const response = await fetch(path, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        const result = await response.json();
        console.log(result); // Stampa la risposta del server
        setIsRoomOccupied(false);
        setOccupant(null);
        alert(result); // Mostra un messaggio all'utente
      } else {
        console.error("Errore nella richiesta:", response.statusText);
        alert("Errore nella richiesta: " + response.statusText);
      }
    } catch (error) {
      console.error("Errore:", error);
      alert("Errore: " + (error as Error).message);
    }
  }

  return (
    <div className="RoomStatus">
      <h2>Stato della stanza</h2>
      <p>La stanza è attualmente {isRoomOccupied ? "occupata" : "libera"}.</p>
      {occupant && <p>La stanza è occupata da: {occupant}</p>}
      {isRoomOccupied ? (
        occupant === username && (
          <button onClick={freeRoom}>Libera la stanza</button>
        )
      ) : (
        <button onClick={occupyRoom}>Occupa la stanza</button>
      )}
      <h3>Classifica</h3>
      <Leaderboard users={leaderboard} />
    </div>
  );
}

export default RoomStatus;
