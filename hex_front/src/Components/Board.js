import React, { useState } from 'react';
import Square from './Square';

export default function Board(props) {

    let handleClickBrd = (idX, idY) => {
        if (props.game.player === props.playerID) {
            props.setCanPlay(false);
            fetch(`${process.env.REACT_APP_RASPBERRY || ""}/play`, {
                method: 'POST',
                body: JSON.stringify({ player: props.game.player, x: idX, y: idY })
            })
                .then(res => res.json())
                .then(result => {
                    props.setGame(result);
                })
                .finally(() => props.setCanPlay(true))
        }
    }

    return (
        <div>
            C'est au tour du <span style={{ color: (props.game.player === 1) ? "lawngreen" : "red" }}> joueur {props.game.player} </span>
            {
                props.game.board.map((valX, indexX) => {
                    return <div key={indexX}
                        style={{ paddingLeft: 60 * indexX }}>
                        {valX.map((valY, indexY) => <Square
                            key={indexY}
                            idxX={indexX}
                            idxY={indexY}
                            handleClickBrd={handleClickBrd}
                            canPlay={props.canPlay}
                            player={valY ? valY.player : undefined} />
                        )}
                    </div>
                }
                )
            }
        </ div>
    );

}