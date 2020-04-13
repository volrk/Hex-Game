import * as React from 'react';
import './Square.css';

export default class Square extends React.Component {
    handleClick() {
        fetch(`${process.env.REACT_APP_RASPBERRY || ""}/play`, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ player: 1, x: this.props.idxX, y: this.props.idxY })
        });
    }

    render() {
        return (
            <button className="square" onClick={() => this.handleClick()}>
                X: {this.props.idxX} - Y: {this.props.idxY}
            </button >
        );
    }
}