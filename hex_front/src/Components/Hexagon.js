import React, { useState } from 'react';
import './Hexagon.css';

export default function Hexagon(props) {

    let handleClickHexagon = () => {
        if (!props.player && props.canPlay) {
            props.handleClickBrd(props.idxX, props.idxY);
        }
    }

    return (
        <div className={`hexagon player${props.player}`} onClick={() => { handleClickHexagon() }}/>
    );

}