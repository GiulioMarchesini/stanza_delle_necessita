import React, { useState } from "react";
import Leaderboard from "./Leaderboard";

const url = "http://127.0.0.1:8080";

interface RoomStatusProps {
  username: string;
}

function RoomStatus({ username }: RoomStatusProps) {
  // TODO fetch di stato stanza e chi la occupa
  const [isRoomOccupied, setIsRoomOccupied] = useState(false);
  const [occupant, setOccupant] = useState<string | null>(null);
  const [timeOfOccupation, setTimeOfOccupation] = useState<number>(0);

  async function occupyRoom() {
    const path: string = url + "/occupy_room";
    const time = Math.floor(new Date().getTime() / 1000);
    setTimeOfOccupation(time);
    const data = { username: username, total_time: time };

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
    setTimeOfOccupation(time - timeOfOccupation);
    // const minuti = Math.floor((time - timeOfOccupation) / 60);
    const minuti = (time - timeOfOccupation) % 60;
    console.log(minuti);
    const data = { username: username, total_time: minuti };

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
      {isRoomOccupied && occupant && (
        <>
          <p>La stanza è occupata da: {occupant}</p>
          <button onClick={freeRoom}>Libera la stanza</button>
        </>
      )}
      {!isRoomOccupied && (
        <button onClick={occupyRoom}>Occupa la stanza</button>
      )}
      <h3>Classifica</h3>
      <Leaderboard />
    </div>
  );
}

export default RoomStatus;
