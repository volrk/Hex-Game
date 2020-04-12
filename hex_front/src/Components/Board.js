import * as React from 'react';
import Square from './Square';

export default class Board extends React.Component {
    render() {
        return (
            <div>
                {this.props.game.board.map((i) => {
                    return <div>
                        {i.map(() => <Square />)}
                    </div>
                }
                )
                }
            </div>
        );
    }
}