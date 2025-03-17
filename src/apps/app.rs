
use super::lib::*;

#[function_component(App)]
pub fn app() -> Html {
    let calculation = use_state(Calculation::new);

    // styles
    let input_button = classes!(input_button());
    html! {
        <div>
            <Global css={global_style()}/>
            
            <div class={classes!(calculator_style())}>
                <div class={classes!(display_style())}>
                    {calculation.display.clone()}
                </div>
                <div>
                    <div>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(7.0))
                        }>{"7"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(8.0))
                        }>{"8"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(9.0))
                        }>{"9"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(15.0))
                        }>{"÷"}</button>
                    </div>
                    <div>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(4.0))
                        }>{"4"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(5.0))
                        }>{"5"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(6.0))
                        }>{"6"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(14.0))
                        }>{"×"}</button>
                    </div>
                    <div>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(1.0))
                        }>{"1"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(2.0))
                        }>{"2"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(3.0))
                        }>{"3"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(13.0))
                        }>{"-"}</button>
                    </div>
                    <div>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(10.0))
                        }>{"C"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(0.0))
                        }>{"0"}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(11.0))
                        }>{"="}</button>
                        <button class={input_button.clone()} onclick={let calculation = calculation.clone();
                            move |_| calculation.set(calculation.input(12.0))
                        }>{"+"}</button>
                    </div>
                </div>
            </div>
        </div>
        
    }
}
