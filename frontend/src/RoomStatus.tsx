import React, { useState, useEffect } from "react";
import Leaderboard from "./Leaderboard";

const url = "http://192.168.0.61:8080";
// TODO ottieni l'url
// TODO metti tutti i tipi in un file a parte
// TODO rendi simpatici gli alert

interface RoomStatusProps {
  username: string;
}

interface User {
  username: string;
  total_time: number;
}

interface RoomState {
  status: string;
  current_user: string | null;
  start_time: number | null; // timestamp in secondi
  end_time: number | null; // timestamp in secondi
}

function RoomStatus({ username }: RoomStatusProps) {
  const [isRoomOccupied, setIsRoomOccupied] = useState(false);
  const [occupant, setOccupant] = useState<string | null>(null);
  const [leaderboard, setLeaderboard] = useState<User[]>([]);

  function ottieniIp() {
    // fetch("https://api64.ipify.org?format=json")
    //   .then((response) => response.json())
    //   .then((data) => {
    //     console.log("Il tuo IP è", data.ip);
    //   })
    //   .catch((error) => {
    //     console.error("Errore:", error);
    //   });
    // TODO ip nella rete locale. es 192.168.0.61
  }

  // fetch di stato stanza e chi la occupa. http request "/room_status"
  async function fetchRoomStatus() {
    const path: string = url + "/room_status";

    try {
      const response = await fetch(path);

      if (response.ok) {
        const result: RoomState = await response.json();
        console.log(result); // Stampa la risposta del server
        setIsRoomOccupied(result.status === "occupata");
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
    const time = Math.floor(new Date().getTime() / 1000);
    const data: RoomState = {
      status: "occupata",
      current_user: username,
      start_time: time,
      end_time: null,
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
      status: "libera",
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
      <button type="button" onClick={ottieniIp}>
        Ip
      </button>
      <h2>Stato della stanza</h2>
      <p>La stanza è attualmente {isRoomOccupied ? "occupata" : "libera"}.</p>
      {isRoomOccupied && occupant === username && (
        <>
          <p>La stanza è occupata da: {occupant}</p>
          <button onClick={freeRoom}>Libera la stanza</button>
        </>
      )}
      {!isRoomOccupied && (
        <button onClick={occupyRoom}>Occupa la stanza</button>
      )}
      <h3>Classifica</h3>
      <Leaderboard users={leaderboard} />
    </div>
  );
}

export default RoomStatus;
