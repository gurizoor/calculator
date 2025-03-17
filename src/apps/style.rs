
use super::lib::*;

pub fn global_style() -> StyleSource {
    css!(
        r#"
            body {
                background: linear-gradient(320deg, rgb(238, 142, 190), rgb(233, 235, 127), rgb(80, 235, 196));
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                margin: 0;
                padding: 0;
                overflow: hidden;
            }
        "#
    )
}

pub fn input_button() -> Style {
    style!(r#"
        margin-top: 0.4vh;
        margin-bottom: 0.4vh;
        margin-left: 0.4vh;
        margin-right: 0.4vh;
        padding-bottom: 1.7vw;
        padding-top: 1.7vh;
        width: 10vh;
        font-size: 6vh;
        box-sizing: border-box;
        background-color: rgb(255, 255, 255);
        border-radius: 5vh;
        border: 0px;
        color: rgb(0, 0, 0);
        box-shadow: 10px 10px 20px 0px rgba(0, 0, 0, 0.2);
        border-bottom: solid 4px rgb(155, 155, 155);

        &:hover {
            transform: translateY(2px);
            box-shadow: 0px 0px 15px 0px rgba(0, 0, 0, 0.2);
        }

        &:active {
            transform: translateY(4px);
            border-bottom: 0px;
            box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.2);
        }
    "#).unwrap()
}

pub fn display_style() -> Style {
    style!(r#"
        margin-top: 0.4vh;
        margin-bottom: 5vh;
        margin-left: 0.4vh;
        margin-right: 0.4vh;
        padding-right: 2vh;
        padding-top: 1vh;
        padding-bottom: 3vh;
        width: 40vh;
        height: 7vh;
        text-align: right;
        font-size: 6vh;
        border: 0px;
        border-radius: 2.5vh;
        background-color: rgb(255, 255, 255);
        border-bottom: solid 3px rgb(155, 155, 155);
        box-shadow: 10px 10px 20px 0px rgba(0, 0, 0, 0.2);
    "#).unwrap()
}

pub fn calculator_style() -> Style{
    style!(r#"
        margin: 0.1vh;
        padding: 3vh;
        border-radius: 5vh;
        width: 44vh;
        height: 65vh;
        background: rgb(255, 255, 255, 0.6);
        box-shadow: 10px 10px 100px 0px rgba(0, 0, 0, 0.2);
    "#).unwrap()
}
