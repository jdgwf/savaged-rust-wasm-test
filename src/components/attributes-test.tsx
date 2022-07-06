import * as React from 'react';
import init, { PlayerCharacter } from 'savaged_libs';

import "../scss/attributes-test.scss";
import CharacterGeneratorBase from './character-generator/character-generator';
export default class AttributesTest extends React.Component<IAttributesTestProps, IAttributesTestState> {

    pc: PlayerCharacter | null = null;
    chargenData: any | null = null;
    constructor(props: IAttributesTestProps) {
        super(props);

        this.state = {
            updated: false,
        }

    }

    async componentDidMount() {

      if(!this.chargenData) {
      console.log("Fetching chargen data from savaged.us", new Date())
      let req = await fetch(
          "https://savaged.us/_api/chargen-data",
          {
            method: "post",
          }
        );

        this.chargenData = await req.json();

        console.log("Fetching chargen data complete from savaged.us", new Date())
      }

        if(!this.pc && this.chargenData) {
            init().then(() => {
              let chargenDataString = JSON.stringify(this.chargenData);
              console.log("Creating work character", new Date() );
              this.pc = new PlayerCharacter( chargenDataString );
              this.pc.name = "Testing More!";
              this.pc.uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";

              this.pc.set_attribute_selected_agility(2);
              this.pc.set_attribute_selected_smarts(1);
              this.pc.set_attribute_selected_spirit(2);
              this.pc.set_attribute_selected_strength(2);
              this.pc.set_attribute_selected_vigor(3);

              this.setState({
                  updated: true,
              })

              console.log("Creating work character complete", new Date() );

              console.log("Starting creation of 100,000 PCs", new Date());
              console.log("chargenDataString.length", chargenDataString.length);
              let pc = new PlayerCharacter( chargenDataString );
              for( let x = 1; x < 100001; x++ ) {

                pc.reset();

                pc.name = "Testing #"  + x.toString();
                // pc.uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";

                pc.set_attribute_selected_agility(2);
                pc.set_attribute_selected_smarts(1);
                pc.set_attribute_selected_spirit(2);
                pc.set_attribute_selected_strength(2);
                pc.set_attribute_selected_vigor(3);

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
  <CharacterGeneratorBase
    pc={this.pc}
    onChange={this.updateCharacter}
  />


            </>
        )
    }
}

interface IAttributesTestProps {

}

interface IAttributesTestState {
    updated: boolean;
}