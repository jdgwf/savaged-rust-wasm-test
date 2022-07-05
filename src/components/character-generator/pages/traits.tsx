import * as React from 'react';
import init, { get_dice_value, PlayerCharacter } from 'savaged_libs';
import { makeRange } from '../../../utils/makeRange';


import "../scss/character-generator.scss"
export default class CharacterGeneratorTraits extends React.Component<ICharacterGeneratorTraitsProps, ICharacterGeneratorTraitsState> {

    constructor(props: ICharacterGeneratorTraitsProps) {
        super(props);


    }


    updateAgility = (
      e: React.FormEvent<HTMLSelectElement>,
      pc: PlayerCharacter
    ) => {
      pc.set_attribute_selected_agility(+e.currentTarget.value);
      // setPC(pc);
      this.props.onChange( pc );
    }

    updateSmarts = (
      e: React.FormEvent<HTMLSelectElement>,
      pc: PlayerCharacter
    ) => {
      pc.set_attribute_selected_smarts(  +e.currentTarget.value );
      this.props.onChange( pc );
    }

    updateSpirit = (
        e: React.FormEvent<HTMLSelectElement>,
        pc: PlayerCharacter
      ) => {
        pc.set_attribute_selected_spirit( +e.currentTarget.value );
        this.props.onChange( pc );
      }

      updateStrength = (
        e: React.FormEvent<HTMLSelectElement>,
        pc: PlayerCharacter
      ) => {
        pc.set_attribute_selected_strength( +e.currentTarget.value);
        this.props.onChange( pc );
      }

        updateVigor = (
            e: React.FormEvent<HTMLSelectElement>,
            pc: PlayerCharacter
        ) => {
            pc.set_attribute_selected_vigor( +e.currentTarget.value );
            this.props.onChange( pc );
        }



    render = (): React.ReactNode =>  {

        return (
            <>


        {this.props.pc && this.props.pc.attributes ? (
          <>
<h2>Traits Control</h2>
<ul className="attribute-select">
            <li>
                <label>Agility:&nbsp;
                <select
                  value={this.props.pc.attributes.selected_agility}
                  //@ts-ignore
                  onChange={(e) => this.updateAgility( e, this.props.pc )}
                >
                  {makeRange(this.props.pc.attributes.min_agility, this.props.pc.attributes.max_agility).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.props.pc.attributes.boosted_agility}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Smarts:&nbsp;
              <select
                  value={this.props.pc.attributes.selected_smarts}
                  //@ts-ignore
                  onChange={(e) => this.updateSmarts( e, this.props.pc )}
                >
                  {makeRange(this.props.pc.attributes.min_smarts, this.props.pc.attributes.max_smarts).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.props.pc.attributes.boosted_smarts}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Spirit:&nbsp;
              <select
                  value={this.props.pc.attributes.selected_spirit}
                  //@ts-ignore
                  onChange={(e) => this.updateSpirit( e, this.props.pc )}
                >
                  {makeRange(this.props.pc.attributes.min_spirit, this.props.pc.attributes.max_spirit).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.props.pc.attributes.boosted_spirit}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Strength:&nbsp;
              <select
                  value={this.props.pc.attributes.selected_strength}
                  //@ts-ignore
                  onChange={(e) => this.updateStrength( e, this.props.pc )}
                >
                  {makeRange(this.props.pc.attributes.min_strength, this.props.pc.attributes.max_strength).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.props.pc.attributes.boosted_strength}>{get_dice_value(value, 0)}</option>
                    )
                  })}
                </select>
              </label>
            </li>
            <li>
              <label>Vigor:&nbsp;
              <select
                  value={this.props.pc.attributes.selected_vigor}
                  //@ts-ignore
                  onChange={(e) => this.updateVigor( e, this.props.pc )}
                >
                  {makeRange(this.props.pc.attributes.min_vigor, this.props.pc.attributes.max_vigor).map( (value: number) => {
                    return (
                      //@ts-ignore
                      <option key={value} value={value - this.props.pc.attributes.boosted_vigor}>{get_dice_value(value, 0)}</option>
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

interface ICharacterGeneratorTraitsProps {
    pc: PlayerCharacter;
    onChange(pc: PlayerCharacter): void;
}

interface ICharacterGeneratorTraitsState {
}