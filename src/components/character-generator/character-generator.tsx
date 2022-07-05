import * as React from 'react';
import init, { get_dice_value, PlayerCharacter } from 'savaged_libs';
import { makeRange } from '../../utils/makeRange';
import SanitizedHTML from '../sanitized_html';
import CharacterGeneratorConcept from './pages/concept';
import CharacterGeneratorTraits from './pages/traits';

import "./scss/character-generator.scss"
export default class CharacterGeneratorBase extends React.Component<ICharacterGeneratorBaseProps, ICharacterGeneratorBaseState> {

    constructor(props: ICharacterGeneratorBaseProps) {
        super(props);

        this.state = {
            updated: false,
        }

    }

    render = (): React.ReactNode =>  {

        return (
            <>

<div style={{display: "flex", width: "100%"}}>
<div style={{ width: "50%"}}>
<CharacterGeneratorConcept
    pc={this.props.pc}
    onChange={this.props.onChange}
/>

<CharacterGeneratorTraits
    pc={this.props.pc}
    onChange={this.props.onChange}
/>
</div>
<div style={{width: "50%"}}>
<h2>export_html()</h2>
<SanitizedHTML
    html={this.props.pc.export_html()}
    raw={true}
/>
</div>
</div>
            </>
        )
    }
}

interface ICharacterGeneratorBaseProps {
    pc: PlayerCharacter;
    onChange(pc: PlayerCharacter): void;
}

interface ICharacterGeneratorBaseState {
    updated: boolean;
}