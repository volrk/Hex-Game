import React, { useState, useEffect } from 'react';
import Square from './Square';

export default function Board(props) {
    const [PlayerTurn, setPlayerTurn] = useState(1);

    let handleClickBrd = (idX, idY) => {
        fetch(`${process.env.REACT_APP_RASPBERRY || ""}/play`, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ player: PlayerTurn, x: idX, y: idY })
        })
            .then(res => res.json())
            .then(result => {
                return props.setGame(result);
            })
        setPlayerTurn(3 - PlayerTurn);
        // this.forceUpdate();
    }

    return (
        <div>
            C'est au tour du joueur {PlayerTurn}
            {
                props.game.board.map((valX, indexX) => {
                    return <div
                        style={{ paddingLeft: 60 * indexX }}>
                        {valX.map((valY, indexY) => <Square
                            idxX={indexX}
                            idxY={indexY}
                            PlayerTurn={PlayerTurn}
                            handleClickBrd={handleClickBrd}
                            player={valY ? valY.player : undefined} />
                        )}
                    </div>
                }
                )
            }
        </ div>
    );

}