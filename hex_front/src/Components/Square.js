import React, { useState } from 'react';
import './Square.css';

export default function Square(props) {

    let handleClickSquare = () => {
        if (!props.player && props.canPlay) {
            props.handleClickBrd(props.idxX, props.idxY);
        }
    }

    return (
        <button className={"square player" + props.player} onClick={() => { handleClickSquare() }}>
        </button >
    );

}