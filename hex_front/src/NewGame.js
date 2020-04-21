import React, { useState } from 'react';
import Board from './Components/Board'

export default function NewGame() {
  const [game, setGame] = useState(null);
  const [init, setInit] = useState(false);

  let handleClickNewGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/new/11`,
    {
      method: "GET",
    }
  )
    .then(res => res.json())
    .then(result => {
      setGame(result); setInit(true);
    });

  let handleClickJoinGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/get`,
    {
      method: "GET",
    }
  )
    .then(res => res.json())
    .then(result => {
      setGame(result); setInit(true);
    });

  if (!init) {
    return (<div>
      <button onClick={() => { handleClickNewGame() }}>
        Nouveau jeu
      </button>

      <button onClick={() => { handleClickJoinGame() }}>
        Rejoindre un jeu
      </button >
    </div>)
  }

  if (game === null) {
    return <div> "Empty game" </div>
  }

  return < Board game={game} setGame={setGame} />
}