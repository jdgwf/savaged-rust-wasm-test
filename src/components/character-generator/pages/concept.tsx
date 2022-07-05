import * as React from 'react';
import init, { get_dice_value, PlayerCharacter } from 'savaged_libs';


import "../scss/character-generator.scss"
export default class CharacterGeneratorConcept extends React.Component<ICharacterGeneratorConceptProps, ICharacterGeneratorConceptState> {

    constructor(props: ICharacterGeneratorConceptProps) {
        super(props);

    }



setName = (
  e: React.FormEvent<HTMLInputElement>,
  pc: PlayerCharacter
) => {
  pc.name = e.currentTarget.value;
  this.props.onChange( pc );
}


    render = (): React.ReactNode =>  {

        return (
            <>


        {this.props.pc && this.props.pc.attributes ? (
          <>
          <h2>Concept Control</h2>
          <label>
            Set Name:<br />
            <input
              type="text"
              value={this.props.pc.name}
              //@ts-ignore
              onChange={(e) => this.setName(e, this.props.pc)}
            />
          </label><br />

          </>
        ) : null}

            </>
        )
    }
}

interface ICharacterGeneratorConceptProps {
    pc: PlayerCharacter;
    onChange(pc: PlayerCharacter): void;
}

interface ICharacterGeneratorConceptState {
}