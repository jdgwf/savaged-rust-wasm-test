import * as React from 'react';
import init, { get_dice_value, PlayerCharacter } from 'savaged_libs';

import { makeRange } from '../utils/makeRange';
import "../scss/attributes-test.scss"
import CharacterGeneratorBase from './character-generator/character-generator';
export default class AttributesTest extends React.Component<IAttributesTestProps, IAttributesTestState> {

    pc: PlayerCharacter | null = null;
    constructor(props: IAttributesTestProps) {
        super(props);

        this.state = {
            updated: false,
        }

    }

    componentDidMount() {
        if(!this.pc) {
            init().then(() => {

                this.pc = new PlayerCharacter();
                this.pc.name = "Testing More!";
                this.pc.uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";

                this.pc.set_attribute_selected_agility(2);
                this.pc.set_attribute_selected_smarts(1);
                this.pc.set_attribute_selected_spirit(2);
                this.pc.set_attribute_selected_strength(2);
                this.pc.set_attribute_selected_vigor(3);

                // this.pc.set_attribute_boosted_vigor(1);
                this.setState({
                    updated: true,
                })

                console.log("Starting creation of 100,000 PCs", new Date());
                for( let x = 1; x < 100001; x++ ) {
                  let pc = new PlayerCharacter();
                  pc.name = "Testing #"  + x.toString();
                  pc.uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";

                  pc.set_attribute_selected_agility(2);
                  pc.set_attribute_selected_smarts(1);
                  pc.set_attribute_selected_spirit(2);
                  pc.set_attribute_selected_strength(2);
                  pc.set_attribute_selected_vigor(3);

                  // pc.set_attribute_boosted_vigor(1);

                  pc.calc();

                  if( x % 10000 === 0)
                    console.log("PC", pc.name );
                }
                console.log("End 100,000 PCs", new Date());
              })

        }
    }

    updateCharacter = (
        pc: PlayerCharacter
    ) => {
        this.pc = pc;
        this.setState({
            updated: true,
        })
    }


    render = (): React.ReactNode =>  {

        return (
            <>
{this.pc ? (
  <CharacterGeneratorBase
    pc={this.pc}
    onChange={this.updateCharacter}
  />
) : null}


            </>
        )
    }
}

interface IAttributesTestProps {

}

interface IAttributesTestState {
    updated: boolean;
}