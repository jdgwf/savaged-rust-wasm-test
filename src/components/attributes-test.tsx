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

              console.log("# Available Books", this.pc.get_available_books_count() );
              console.log("get_available_books", JSON.parse( this.pc.get_available_books() ) );

              console.log("# Available Edges", this.pc.get_available_edges_count() );
              console.log("get_available_edges", JSON.parse( this.pc.get_available_edges() ) );

              console.log("# Available Hindrances", this.pc.get_available_hindrances_count() )
              console.log("get_available_hindrances", JSON.parse( this.pc.get_available_hindrances() ) );

              console.log("# Available Gear", this.pc.get_available_gear_count() );
              console.log("get_available_gear", JSON.parse( this.pc.get_available_gear() ) );

              console.log("# Available Weapons", this.pc.get_available_weapon_count() );
              console.log("get_available_weapons", JSON.parse( this.pc.get_available_weapons() ) );

              console.log("# Available Armor", this.pc.get_available_armor_count() );
              console.log("get_available_armor", JSON.parse( this.pc.get_available_armor() ) );

              this.setState({
                  updated: true,
              })

              console.log("Creating work character complete", new Date() );

              let num_test_pcs = 100000;
              console.log("Starting creation of " + num_test_pcs + " PCs", new Date());
              console.log("chargenDataString.length", chargenDataString.length);

              let pc = new PlayerCharacter( chargenDataString );
              for( let x = 1; x < num_test_pcs + 1; x++ ) {
                // this is an EXPENSIVE operation... it's easier to reuse a created object and reimport if possible. (as above - it's - at least - 1000x faster)
                // let pc = new PlayerCharacter( chargenDataString );
                pc.reset();

                pc.name = "Testing #"  + x.toString();
                // pc.uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";

                pc.set_attribute_selected_agility(2);
                pc.set_attribute_selected_smarts(1);
                pc.set_attribute_selected_spirit(2);
                pc.set_attribute_selected_strength(2);
                pc.set_attribute_selected_vigor(3);

                pc.calc();

                if( x % (num_test_pcs / 10) === 0)
                  console.log("PC", pc.name );
              }
              console.log("End " + num_test_pcs + " PCs", new Date());
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