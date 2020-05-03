import React, { useState, useEffect } from 'react';
import Board from './Components/Board'
import Popup from "reactjs-popup";

export default function NewGame() {
  const [game, setGame] = useState(null);
  const [init, setInit] = useState(false);
  const [playerID, setPlayerId] = useState(null);
  const [canPlay, setCanPlay] = useState(false);
  const [toogleTimer, setToogleTimer] = useState(false);
  const [gameId, setGameId] = useState(null);
  const [boardSize, setBoardSize] = useState(null);

  let handleClickNewGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/new/${boardSize}`,
    {
      method: "GET",
    })
    .then(res => res.json())
    .then(result => {
      setGame(result);
      setPlayerId(result.player);
      setInit(true);
      setCanPlay(true);
      setGameId(result.id);
    }
    );

  let updateGame = () => fetch(
    `${process.env.REACT_APP_RASPBERRY || ""}/hex/get/${gameId}`,
    {
      method: "GET",
    })
    .then(res => res.json())
    .then(result => {
      setGame(result);
    }
    );

  useEffect(() => {
    const timer = setTimeout(async () => {
      try {
        if (init && (!game.winner)) {
          await updateGame();
        }
      }
      finally {
        setToogleTimer(!toogleTimer);
      }
    }, 1000);
    return () => { clearTimeout(timer); };
  }, [toogleTimer]);

  let handleClickJoinGame = async () => {
    try {
      await updateGame();
      setInit(true); setCanPlay(true);
    }
    catch (err) {
    }
  };

  if (!init) {
    return (
      <div>
        <Popup
          trigger={<button className="button"> Nouveau jeu</button>}
          modal
          closeOnDocumentClick>
          <div className="modal">
            <div>
              <label>Taille du jeu : </label>
              <input type="number" id="boardId" min="1" max="15" onChange={event => setBoardSize(event.target.value)} />
            </div>
            <button onClick={() => { handleClickNewGame() }}>Nouveau</button >
          </div>
        </Popup>
        <Popup
          trigger={<button className="button"> Rejoindre un jeu </button>}
          modal
          closeOnDocumentClick>
          <div className="modal">
            <div>
              <label>Partie : </label>
              <input type="number" id="boardId" min="0" max="9" onChange={event => setGameId(event.target.value)} />
            </div>
            <div>
              <label>Joueur : </label>
              <input type="radio" id="1" name="playeur" checked={playerID === 1} onChange={() => setPlayerId(1)} />
              <label>1</label>
              <input type="radio" id="2" name="playeur" checked={playerID === 2} onChange={() => setPlayerId(2)} />
              <label>2</label>
            </div>
            <button onClick={() => { handleClickJoinGame() }}>Rejoindre</button >
          </div>
        </Popup>
      </div>)
  }

  if (game === null) {
    return <div> "Empty game" </div>
  }

  return (<div>
    Tu es le <b>joueur {playerID} </b> et tu joues à la partie <b>n° {game.id} </b>
    < Board game={game} setGame={setGame} playerID={playerID} setCanPlay={setCanPlay} canPlay={canPlay} />
  </div>)
}