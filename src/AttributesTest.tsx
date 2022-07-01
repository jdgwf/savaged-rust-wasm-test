import * as React from 'react';
import init, { get_dice_value, PlayerCharacter } from 'savagedlib';

import { makeRange } from './utils/makeRange';

export default class Moo extends React.Component<IMooProps, IMooState> {

    pc: PlayerCharacter | null = null;
    constructor(props: IMooProps) {
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

                this.pc.set_attribute_boosted_vigor(1);
                this.setState({
                    updated: true,
                })
              })

        }
    }


updateAgility = (
    e: React.FormEvent<HTMLSelectElement>,
    pc: PlayerCharacter
  ) => {
    console.log("XX", +e.currentTarget.value);
    pc.set_attribute_selected_agility(+e.currentTarget.value);
    // setPC(pc);
    this.setState({
        updated: true,
    })
  }

  updateSmarts = (
    e: React.FormEvent<HTMLSelectElement>,
    pc: PlayerCharacter
  ) => {
    pc.set_attribute_selected_smarts(  +e.currentTarget.value );
    this.setState({
        updated: true,
    })
  }

  setName = (
    e: React.FormEvent<HTMLInputElement>,
    pc: PlayerCharacter
  ) => {
    pc.name = e.currentTarget.value;
    this.setState({
        updated: true,
    })
  }

updateSpirit = (
    e: React.FormEvent<HTMLSelectElement>,
    pc: PlayerCharacter
  ) => {
    pc.set_attribute_selected_spirit( +e.currentTarget.value );
    this.setState({
        updated: true,
    })
  }

  updateStrength = (
    e: React.FormEvent<HTMLSelectElement>,
    pc: PlayerCharacter
  ) => {
    pc.set_attribute_selected_strength( +e.currentTarget.value);
    this.setState({
        updated: true,
    })
  }

    updateVigor = (
        e: React.FormEvent<HTMLSelectElement>,
        pc: PlayerCharacter
    ) => {
        pc.set_attribute_selected_vigor( +e.currentTarget.value );
        this.setState({
            updated: true,
        })
    }


    render = (): React.ReactNode =>  {

        return (
            <>

        <strong>PC Name</strong>: {this.pc && this.pc.name ? this.pc.name : ""}<br />

        <strong>UUID</strong>: {this.pc && this.pc.uuid ? this.pc.uuid : ""}<br />
        <ul>
          <li><strong>Agility</strong>: {this.pc && this.pc.attributes && this.pc.attributes.agility_hr ? this.pc.attributes.agility_hr : "0" }</li>
          <li><strong>Smarts</strong>: {this.pc && this.pc.attributes && this.pc.attributes.smarts_hr ? this.pc.attributes.smarts_hr : "0" }</li>
          <li><strong>Spirit</strong>: {this.pc && this.pc.attributes && this.pc.attributes.spirit_hr ? this.pc.attributes.spirit_hr : "0" }</li>
          <li><strong>Strength</strong>: {this.pc && this.pc.attributes && this.pc.attributes.strength_hr ? this.pc.attributes.strength_hr : "0" }</li>
          <li><strong>Vigor</strong>: {this.pc && this.pc.attributes && this.pc.attributes.vigor_hr ? this.pc.attributes.vigor_hr : "0" }</li>
        </ul>

        {this.pc && this.pc.attributes ? (
          <>
          <h2>Presto-Chango Controls</h2>
          <label>
            Set Name:<br />
            <input
              type="text"
              value={this.pc.name}
              //@ts-ignore
              onChange={(e) => this.setName(e, this.pc)}
            />
          </label>
          <ul>
            <li>
                <label>Agility:&nbsp;
                <select
                  value={this.pc.attributes.selected_agility}
                  //@ts-ignore
                  onChange={(e) => this.updateAgility( e, this.pc )}
                >
                  {makeRange(this.pc.attributes.min_agility, this.pc.attributes.max_agility).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.pc.attributes.boosted_agility}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Smarts:&nbsp;
              <select
                  value={this.pc.attributes.selected_smarts}
                  //@ts-ignore
                  onChange={(e) => this.updateSmarts( e, this.pc )}
                >
                  {makeRange(this.pc.attributes.min_smarts, this.pc.attributes.max_smarts).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.pc.attributes.boosted_smarts}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Spirit:&nbsp;
              <select
                  value={this.pc.attributes.selected_spirit}
                  //@ts-ignore
                  onChange={(e) => this.updateSpirit( e, this.pc )}
                >
                  {makeRange(this.pc.attributes.min_spirit, this.pc.attributes.max_spirit).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.pc.attributes.boosted_spirit}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Strength:&nbsp;
              <select
                  value={this.pc.attributes.selected_strength}
                  //@ts-ignore
                  onChange={(e) => this.updateStrength( e, this.pc )}
                >
                  {makeRange(this.pc.attributes.min_strength, this.pc.attributes.max_strength).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.pc.attributes.boosted_strength}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Vigor:&nbsp;
              <select
                  value={this.pc.attributes.selected_vigor}
                  //@ts-ignore
                  onChange={(e) => this.updateVigor( e, this.pc )}
                >
                  {makeRange(this.pc.attributes.min_vigor, this.pc.attributes.max_vigor).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.pc.attributes.boosted_vigor}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
          </ul>
          </>
        ) : null}

            </>
        )
    }
}

interface IMooProps {

}

interface IMooState {
    updated: boolean;
}