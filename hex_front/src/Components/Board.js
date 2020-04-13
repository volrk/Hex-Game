import * as React from 'react';
import Square from './Square';

export default class Board extends React.Component {
    render() {
        return (
            <div>
                {this.props.game.board.map((valX, indexX) => {
                    return <div
                        style={{ paddingLeft: 60 * indexX }}>
                        {valX.map((valY, indexY) => <Square idxX={indexX} idxY={indexY} />)}
                    </div>
                }
                )
                }
            </div>
        );
    }
}