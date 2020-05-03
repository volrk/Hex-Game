import React, { useState, useEffect } from 'react';
import Board from './Components/Board'

export default function NewGame() {
  const [game, setGame] = useState(null);
  const [init, setInit] = useState(false);
  const [playerID, setPlayerId] = useState(null);
  const [canPlay, setCanPlay] = useState(false);
  const [toogleTimer, setToogleTimer] = useState(false);

  let handleClickNewGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/new/5`,
    {
      method: "GET",
    }
  )
    .then(res => res.json())
    .then(result => {
      setGame(result); setPlayerId(result.player); setInit(true); setCanPlay(true);
    });

  let updateGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/get`,
    {
      method: "GET",
    }
  )
    .then(res => res.json())
    .then(result => {
      setGame(result);
    }
    )

  useEffect(() => {
    const timer = setTimeout(async () => {
      await updateGame();
      setToogleTimer(!toogleTimer);
    }, 1000);
    return () => { clearTimeout(timer); };
  }, [toogleTimer]);

  let handleClickJoinGame = () => {
    updateGame();
    setPlayerId(2); setInit(true); setCanPlay(true);
  };

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

  return (<div>
    Tu es le <b>joueur {playerID} </b>
    < Board game={game} setGame={setGame} playerID={playerID} setCanPlay={setCanPlay} canPlay={canPlay} />
  </div>)
}