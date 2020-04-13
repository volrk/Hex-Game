import React, { useState, useEffect } from 'react';
import './Square.css';

export default function Square(props) {
    const [player, setPlayer] = useState(props.player);

    let handleClickSquare = () => {
        if (!player) {
            setPlayer(props.PlayerTurn);
            props.handleClickBrd(props.idxX, props.idxY);
        }
    }

    return (
        <button className={"square player" + props.player} onClick={() => { handleClickSquare() }}>
            X: {props.idxX} - Y: {props.idxY}
        </button >
    );

}