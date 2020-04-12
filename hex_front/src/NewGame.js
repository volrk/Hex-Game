import React, { useState, useEffect } from 'react';
import Board from './Components/Board'

export default function NewGame() {
  const [game, setGame] = useState(null);

  useEffect(() => {
    fetch(
      `${process.env.REACT_APP_RASPBERRY || ""}/hex/new/3`,
      {
        method: "GET",
      }
    )
      .then(res => res.json())
      .then(result => {
        return setGame(result);
      })
  },
    []);
  if (game === null) {
    return <div> "Empty game" </div>
  }

  return <Board game={game} />;
}