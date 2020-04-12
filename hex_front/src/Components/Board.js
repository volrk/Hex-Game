import * as React from 'react';
import Square from './Square';

export default class Board extends React.Component {
    render() {
        return (
            <div>
                {this.props.game.board.map((val, index) => {
                    return <div
                        style={{ paddingLeft: 60 * index }}>
                        {val.map(() => <Square />)}
                    </div>
                }
                )
                }
            </div>
        );
    }
}