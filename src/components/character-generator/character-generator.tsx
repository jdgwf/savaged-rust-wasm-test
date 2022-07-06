import * as React from 'react';
import { PlayerCharacter } from 'savaged_libs';
import SanitizedHTML from '../sanitized_html';
import CharacterGeneratorConcept from './pages/concept';
import CharacterGeneratorTraits from './pages/traits';

import "./scss/character-generator.scss";
export default class CharacterGeneratorBase extends React.Component<ICharacterGeneratorBaseProps, ICharacterGeneratorBaseState> {

    constructor(props: ICharacterGeneratorBaseProps) {
        super(props);

        this.state = {
            updated: false,
        }

    }

    render = (): React.ReactNode =>  {

        if(!this.props.pc) {
            return <>Loading...</>
        }
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
<h2>export_html() (from rust wasm struct)</h2>
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
    pc: PlayerCharacter | null;
    onChange(pc: PlayerCharacter): void;
}

interface ICharacterGeneratorBaseState {
    updated: boolean;
}