import React, { useState } from 'react';
import Hexagon from './Hexagon';
import Popup from "reactjs-popup";
import './Board.css';

export default function Board(props) {

    let handleClickBrd = (idX, idY) => {
        if (props.game.player === props.playerID) {
            props.setCanPlay(false);
            fetch(`${process.env.REACT_APP_RASPBERRY || ""}/hex/play/${props.game.id}`, {
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

    let getPlayeur2Line = (board) => {
        let row = [];
        board[0].forEach((element, index) => {
            row.push(<Hexagon
                key={index}
                player={2} />)
        });
        return row;
    }

    let boardWidth = 76 * (props.game.board.length + (props.game.board.length - 1) / 2 + 2);

    return (
        <div>
            C'est au tour du <span style={{ color: (props.game.player === 1) ? "lawngreen" : "red" }}> joueur {props.game.player} </span>
            <div className={"board"} style={{ width: boardWidth, marginLeft: `calc(calc(100% - ${boardWidth}px)/2)` }}>
                <div className={"row"} style={{ paddingLeft: 38 }}>
                    {getPlayeur2Line(props.game.board)}
                </div>
                {
                    props.game.board.map((valX, indexX) => {
                        return <div key={indexX}
                            className={"row"}
                            style={{ paddingLeft: 38 * indexX }}>
                            <Hexagon player={1} />
                            {valX.map((valY, indexY) => <Hexagon
                                key={indexY}
                                idxX={indexX}
                                idxY={indexY}
                                handleClickBrd={handleClickBrd}
                                canPlay={props.canPlay}
                                player={valY ? valY.player : undefined} />
                            )}
                            <Hexagon player={1} />
                        </div>
                    })
                }
                <div className={"row"} style={{ paddingLeft: 38 * (props.game.board.length + 2) }}>
                    {getPlayeur2Line(props.game.board)}
                </div>
            </div>
            <Popup
                open={props.game.winner != null}
                closeOnDocumentClick>
                <div >
                    <span style={{ color: (props.game.winner === 1) ? "lawngreen" : "red" }}> <b>Joueur {props.game.winner} a gagné </b></span>
                </div>
            </Popup>
        </ div>
    );

}